// Example Rhai custom-code mod for Star Wars Episode I (pin2k-core).
// Demonstrates modes, timers, per-player variables, switch interception, and scoring.

const SW_CAPTIVE_BALL = 8;
const SW_SNEAKY_LANE = 15;
const FL_JET = 20;
const LAMP_EXTRA_BALL = 79;

fn on_ball_start() {
    set_var("hurry_up_value", 500_000);
}

fn on_switch(sw_id) {
    if sw_id == SW_SNEAKY_LANE && !is_mode_active("droid_hurry_up") {
        start_mode("droid_hurry_up", 15_000);
        set_var("hurry_up_value", 500_000);
        set_lamp(LAMP_EXTRA_BALL, true);
        set_banner("DROID HURRY UP! HIT CAPTIVE BALL!", 2500);
        // Let base Sneaky Lane / Skill Shot rule run as well
        return false;
    }

    if sw_id == SW_CAPTIVE_BALL && is_mode_active("droid_hurry_up") {
        let award = get_var("hurry_up_value");
        add_score(award);
        stop_mode("droid_hurry_up");
        set_lamp(LAMP_EXTRA_BALL, false);
        fire_coil(FL_JET, 32);
        set_banner("DROID HURRY UP COLLECTED! +" + award, 3000);
        // Suppress the base Captive Ball handler for this shot
        return true;
    }

    false
}

fn on_tick(dt_ms) {
    if is_mode_active("droid_hurry_up") {
        let val = get_var("hurry_up_value") - (dt_ms * 10);
        if val < 100_000 {
            val = 100_000;
        }
        set_var("hurry_up_value", val);
    }
}

fn on_mode_exit(name) {
    if name == "droid_hurry_up" {
        set_lamp(LAMP_EXTRA_BALL, false);
        set_banner("DROID HURRY UP EXPIRED", 1500);
    }
}
