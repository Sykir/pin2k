//! Sandboxed Rhai custom-code mod runtime (`*.rhai`) for `pin2k-core`.
//!
//! Mods can provide a `main.rhai` script via `mod.json` (`"script": "main.rhai"`)
//! implementing any subset of lifecycle hooks:
//! - `on_init()`
//! - `on_ball_start()`
//! - `on_ball_end()`
//! - `on_switch(sw_id)` -> return `true` (or call `override_default()`) to suppress base game rule
//! - `on_tick(dt_ms)`
//! - `on_timer(name)`
//! - `on_mode_exit(name)`
//! - `on_tilt()`

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use rhai::{Dynamic, Engine, FuncArgs, Scope, AST};

/// Side-effect command queued by a Rhai mod script and executed by `Pin2kMachine`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModCommand {
    AddScore(u64),
    FireCoil { id: u8, pulse_ms: u8 },
    SetHold { id: u8, on: bool },
    SetLamp { id: u8, on: bool },
    PlaySound(u16),
    PlayArd(String),
    SetBanner { text: String, ms: u32 },
    ShowAnim(String),
    EjectBall,
}

/// Read-only machine state snapshot visible to Rhai mod scripts.
#[derive(Debug, Clone, Default)]
pub struct ModSnapshot {
    pub current_player: usize,
    pub current_ball: u8,
    pub score: u64,
    pub balls_in_play: u8,
    pub playfield_valid: bool,
    pub in_scene: bool,
    pub in_multiball: bool,
}

#[derive(Debug, Default)]
struct ModScriptState {
    snapshot: ModSnapshot,
    vars: BTreeMap<String, i64>,
    player_vars: [BTreeMap<String, i64>; 4],
    timers: BTreeMap<String, u32>,
    modes: BTreeMap<String, Option<u32>>,
    pending_cmds: Vec<ModCommand>,
    override_default: bool,
}

/// Compiled Rhai mod script runtime with sandboxed operation limits.
pub struct ModScriptRuntime {
    engine: Engine,
    ast: AST,
    scope: Scope<'static>,
    state: Arc<Mutex<ModScriptState>>,
}

impl std::fmt::Debug for ModScriptRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModScriptRuntime").finish_non_exhaustive()
    }
}

impl ModScriptRuntime {
    /// Compiles a Rhai script source string and registers the Pinball 2000 Mod API.
    pub fn compile(source: &str) -> Result<Self, String> {
        let state = Arc::new(Mutex::new(ModScriptState::default()));
        let mut engine = Engine::new();
        // Prevent infinite loops in mod scripts from stalling the 60 Hz machine loop.
        engine.set_max_operations(50_000);

        register_mod_api(&mut engine, &state);

        let ast = engine.compile(source).map_err(|e| e.to_string())?;
        let mut scope = Scope::new();
        // Evaluate top-level constants/statements once.
        let _ = engine.run_ast_with_scope(&mut scope, &ast).map_err(|e| e.to_string())?;

        Ok(Self {
            engine,
            ast,
            scope,
            state,
        })
    }

    /// Calls an optional hook function in the Rhai script if defined.
    /// Returns `(override_default, queued_commands)`.
    pub fn call_hook(
        &mut self,
        fn_name: &str,
        args: impl FuncArgs,
        snapshot: ModSnapshot,
    ) -> (bool, Vec<ModCommand>) {
        {
            let mut st = self.state.lock().unwrap();
            st.snapshot = snapshot;
            st.override_default = false;
            st.pending_cmds.clear();
        }

        let mut returned_override = false;
        if self.has_fn(fn_name) {
            if let Ok(val) = self.engine.call_fn::<Dynamic>(&mut self.scope, &self.ast, fn_name, args) {
                if let Ok(b) = val.as_bool() {
                    returned_override = b;
                }
            }
        }

        let mut st = self.state.lock().unwrap();
        let override_default = returned_override || st.override_default;
        let cmds = std::mem::take(&mut st.pending_cmds);
        (override_default, cmds)
    }

    /// Advances mod timers and timed modes by `dt_ms`, returning `(expired_timers, expired_modes)`.
    pub fn tick_timers_and_modes(&mut self, dt_ms: u32) -> (Vec<String>, Vec<String>) {
        let mut st = self.state.lock().unwrap();
        let mut expired_timers = Vec::new();
        let mut expired_modes = Vec::new();

        st.timers.retain(|name, rem| {
            *rem = rem.saturating_sub(dt_ms);
            if *rem == 0 {
                expired_timers.push(name.clone());
                false
            } else {
                true
            }
        });

        st.modes.retain(|name, opt_rem| {
            if let Some(rem) = opt_rem {
                *rem = rem.saturating_sub(dt_ms);
                if *rem == 0 {
                    expired_modes.push(name.clone());
                    return false;
                }
            }
            true
        });

        (expired_timers, expired_modes)
    }

    fn has_fn(&self, fn_name: &str) -> bool {
        self.ast.iter_functions().any(|f| f.name == fn_name)
    }
}

fn register_mod_api(engine: &mut Engine, state: &Arc<Mutex<ModScriptState>>) {
    let s = Arc::clone(state);
    engine.register_fn("add_score", move |pts: i64| {
        if pts > 0 {
            s.lock().unwrap().pending_cmds.push(ModCommand::AddScore(pts as u64));
        }
    });

    let s = Arc::clone(state);
    engine.register_fn("fire_coil", move |id: i64, pulse_ms: i64| {
        s.lock().unwrap().pending_cmds.push(ModCommand::FireCoil {
            id: id.clamp(0, 47) as u8,
            pulse_ms: pulse_ms.clamp(1, 255) as u8,
        });
    });

    let s = Arc::clone(state);
    engine.register_fn("set_hold", move |id: i64, on: bool| {
        s.lock().unwrap().pending_cmds.push(ModCommand::SetHold {
            id: id.clamp(0, 47) as u8,
            on,
        });
    });

    let s = Arc::clone(state);
    engine.register_fn("set_lamp", move |id: i64, on: bool| {
        s.lock().unwrap().pending_cmds.push(ModCommand::SetLamp {
            id: id.clamp(0, 127) as u8,
            on,
        });
    });

    let s = Arc::clone(state);
    engine.register_fn("play_sound", move |id: i64| {
        s.lock().unwrap().pending_cmds.push(ModCommand::PlaySound(id.clamp(0, 0xFFFF) as u16));
    });

    let s = Arc::clone(state);
    engine.register_fn("play_ard", move |pool: String| {
        s.lock().unwrap().pending_cmds.push(ModCommand::PlayArd(pool));
    });

    let s = Arc::clone(state);
    engine.register_fn("set_banner", move |text: String, ms: i64| {
        s.lock().unwrap().pending_cmds.push(ModCommand::SetBanner {
            text,
            ms: ms.max(0) as u32,
        });
    });

    let s = Arc::clone(state);
    engine.register_fn("show_anim", move |anim: String| {
        s.lock().unwrap().pending_cmds.push(ModCommand::ShowAnim(anim));
    });

    let s = Arc::clone(state);
    engine.register_fn("eject_ball", move || {
        s.lock().unwrap().pending_cmds.push(ModCommand::EjectBall);
    });

    let s = Arc::clone(state);
    engine.register_fn("override_default", move || {
        s.lock().unwrap().override_default = true;
    });

    // Per-player and global variables
    let s = Arc::clone(state);
    engine.register_fn("get_var", move |name: String| -> i64 {
        let st = s.lock().unwrap();
        let p = st.snapshot.current_player % 4;
        st.player_vars[p].get(&name).copied().unwrap_or(0)
    });

    let s = Arc::clone(state);
    engine.register_fn("set_var", move |name: String, val: i64| {
        let mut st = s.lock().unwrap();
        let p = st.snapshot.current_player % 4;
        st.player_vars[p].insert(name, val);
    });

    let s = Arc::clone(state);
    engine.register_fn("get_global", move |name: String| -> i64 {
        s.lock().unwrap().vars.get(&name).copied().unwrap_or(0)
    });

    let s = Arc::clone(state);
    engine.register_fn("set_global", move |name: String, val: i64| {
        s.lock().unwrap().vars.insert(name, val);
    });

    // Timers & Modes
    let s = Arc::clone(state);
    engine.register_fn("start_timer", move |name: String, ms: i64| {
        if ms > 0 {
            s.lock().unwrap().timers.insert(name, ms as u32);
        }
    });

    let s = Arc::clone(state);
    engine.register_fn("cancel_timer", move |name: String| {
        s.lock().unwrap().timers.remove(&name);
    });

    let s = Arc::clone(state);
    engine.register_fn("start_mode", move |name: String, ms: i64| {
        let dur = if ms > 0 { Some(ms as u32) } else { None };
        s.lock().unwrap().modes.insert(name, dur);
    });

    let s = Arc::clone(state);
    engine.register_fn("stop_mode", move |name: String| {
        s.lock().unwrap().modes.remove(&name);
    });

    let s = Arc::clone(state);
    engine.register_fn("is_mode_active", move |name: String| -> bool {
        s.lock().unwrap().modes.contains_key(&name)
    });

    // Read-only machine snapshot getters
    let s = Arc::clone(state);
    engine.register_fn("player", move || -> i64 {
        (s.lock().unwrap().snapshot.current_player + 1) as i64
    });

    let s = Arc::clone(state);
    engine.register_fn("ball", move || -> i64 {
        s.lock().unwrap().snapshot.current_ball as i64
    });

    let s = Arc::clone(state);
    engine.register_fn("score", move || -> i64 {
        s.lock().unwrap().snapshot.score as i64
    });

    let s = Arc::clone(state);
    engine.register_fn("balls_in_play", move || -> i64 {
        s.lock().unwrap().snapshot.balls_in_play as i64
    });

    let s = Arc::clone(state);
    engine.register_fn("playfield_valid", move || -> bool {
        s.lock().unwrap().snapshot.playfield_valid
    });

    let s = Arc::clone(state);
    engine.register_fn("in_scene", move || -> bool {
        s.lock().unwrap().snapshot.in_scene
    });

    let s = Arc::clone(state);
    engine.register_fn("in_multiball", move || -> bool {
        s.lock().unwrap().snapshot.in_multiball
    });
}
