// Star Wars Episode I — v2.10 Ruleset Mod (Rhai)
// Implements the v2.10 additions documented in work/swe1-regles/livre_regles_swe1_210.md:
// 1. Quick Multiball (Watto on Captive Ball: 4 hits -> 8 hits, 500k + 1M jackpots)
// 2. Droid Hurry Up (Sneaky Lane 5x -> Left Drop Target / Left Ramp, 1M->100k, +250k/win)
// 3. Double Scoring (All Scores X2 for 25s, once per game)
// 4. Midnight Madness (4 balls for 20s, +100k per playfield switch)
// 5. C-3PO Build Champion (cumulative pieces vs 12 default)
// 6. Darth Maul Sudden Death (left flipper warning/loss if < 2 balls after grace)

const SW_LEFT_DROP = 4;
const SW_RIGHT_OUTLANE = 14;
const SW_RIGHT_INLANE = 6;
const SW_CAPTIVE_BALL = 8;
const SW_SNEAKY_LANE = 15;
const SW_LEFT_SAUCER = 22;
const SW_RIGHT_SAUCER = 23;
const SW_LEFT_RAMP_ENTER = 29;
const SW_LEFT_SLING = 40;
const SW_RIGHT_SLING = 41;
const SW_LEFT_LOOP_UPPER = 46;
const SW_LEFT_LOOP_ROLLOVER = 47;
const SW_LEFT_FLIPPER = 85;

const FL_JET = 20;
const LAMP_EXTRA_BALL = 79;

fn on_init() {
    set_global("c3p0_champ_record", 12);
    set_global("midnight_always_on", 0);
}

fn on_ball_start() {
    set_score_multiplier(1);
    if get_var("quick_mb_target") == 0 {
        set_var("quick_mb_target", 4);
    }
    if get_var("droid_hurry_up_left") == 0 {
        set_var("droid_hurry_up_left", 5);
    }
}

fn on_ball_end() {
    stop_mode("double_scoring");
    stop_mode("droid_hurry_up");
    stop_mode("quick_multiball");
    stop_mode("midnight_madness");
    set_score_multiplier(1);
}

fn on_tilt() {
    stop_mode("double_scoring");
    stop_mode("droid_hurry_up");
    stop_mode("quick_multiball");
    stop_mode("midnight_madness");
    set_score_multiplier(1);
}

fn on_switch(sw_id) {
    // Midnight Madness: +100,000 on every playfield switch while active
    if is_mode_active("midnight_madness") && sw_id < 64 {
        add_score(100_000);
    }

    // 1. Quick Multiball (Captive Ball: 15,010 pts in 2.10, 4 then 8 hits -> 2-ball MB + 1M Jackpots)
    if sw_id == SW_CAPTIVE_BALL {
        if is_mode_active("quick_multiball") {
            if balls_in_play() >= 2 {
                add_score(1_000_000);
                fire_coil(FL_JET, 32);
                set_banner("QUICK MULTIBALL JACKPOT! +1,000,000", 2500);
                return true;
            } else {
                stop_mode("quick_multiball");
            }
        }
        if !in_multiball() && !is_mode_active("midnight_madness") {
            add_score(15_010);
            let hits = get_var("quick_mb_hits") + 1;
            let target = get_var("quick_mb_target");
            if target == 0 { target = 4; }
            if hits >= target {
                set_var("quick_mb_hits", 0);
                set_var("quick_mb_target", 8);
                add_score(500_000);
                eject_ball();
                start_mode("quick_multiball", 0);
                set_banner("QUICK MULTIBALL! JACKPOTS 1,000,000!", 3000);
            } else {
                set_var("quick_mb_hits", hits);
                let rem = target - hits;
                set_banner("QUICK MULTIBALL IN " + rem + " HITS", 1800);
            }
            return true;
        }
        return false;
    }

    // 2. Droid Hurry Up (Sneaky Lane 5x -> Left Drop Target / Left Ramp)
    if sw_id == SW_SNEAKY_LANE {
        let rem = get_var("droid_hurry_up_left");
        if rem <= 1 {
            set_var("droid_hurry_up_left", 5);
            let won = get_var("droid_hurry_ups_won");
            let start_val = 1_000_000 + (won * 250_000);
            set_var("droid_hurry_up_val", start_val);
            set_drop_target(false, true);
            fire_coil(FL_JET, 32);
            start_mode("droid_hurry_up", 16_000);
            set_banner("DROID HURRY UP! " + start_val, 2500);
        } else {
            let next_rem = rem - 1;
            set_var("droid_hurry_up_left", next_rem);
            set_banner(next_rem + " MORE FOR DROID HURRY UP", 1800);
        }
        // Let base Sneaky Lane / Skill Shot run as well
        return false;
    }

    if (sw_id == SW_LEFT_DROP || sw_id == SW_LEFT_RAMP_ENTER) && is_mode_active("droid_hurry_up") {
        let award = get_var("droid_hurry_up_val");
        add_score(award);
        set_var("droid_hurry_ups_won", get_var("droid_hurry_ups_won") + 1);
        stop_mode("droid_hurry_up");
        fire_coil(FL_JET, 32);
        set_banner("DROID HURRY UP! +" + award, 3000);
        return false;
    }

    // 3. Double Scoring (2X for 25s, once per game via Saucers when conditions met)
    if sw_id == SW_LEFT_SAUCER || sw_id == SW_RIGHT_SAUCER {
        let shops = get_var("shop_visits") + 1;
        set_var("shop_visits", shops);
        if shops >= 4 && get_var("double_scoring_used") == 0 {
            set_var("double_scoring_used", 1);
            set_score_multiplier(2);
            start_mode("double_scoring", 25_000);
            set_lamp(LAMP_EXTRA_BALL, true);
            set_banner("ALL SCORES (X2) FOR 25 SECONDS!", 3000);
        }
        return false;
    }

    // 4. Midnight Madness trigger (slings, right inlane/outlane, sneaky lane when armed)
    if (sw_id == SW_LEFT_SLING || sw_id == SW_RIGHT_SLING || sw_id == SW_RIGHT_INLANE || sw_id == SW_RIGHT_OUTLANE)
        && !in_scene()
        && !is_mode_active("midnight_madness")
        && get_var("midnight_done") == 0
    {
        if get_global("midnight_always_on") == 1 || get_var("midnight_primed") == 1 {
            set_var("midnight_done", 1);
            set_var("midnight_primed", 0);
            start_mode("midnight_madness", 23_000);
            // Maintain 4 balls in play
            let need = 4 - balls_in_play();
            let i = 0;
            while i < need {
                eject_ball();
                i += 1;
            }
            set_banner("MIDNIGHT MADNESS! +100,000 EVERY SWITCH!", 4000);
        }
        return false;
    }

    // 5. C-3PO Build Champion cumulative counter (never reset in 2.10!)
    if (sw_id == SW_LEFT_LOOP_UPPER || sw_id == SW_LEFT_LOOP_ROLLOVER) && !in_multiball() {
        let key = "c3p0_total_p" + player();
        let total = get_global(key) + 1;
        set_global(key, total);
        let rec = get_global("c3p0_champ_record");
        if total > rec {
            set_global("c3p0_champ_record", total);
            set_global("c3p0_champ_player", player());
            set_banner("NEW C-3PO BUILD CHAMPION: " + total + " PARTS!", 2500);
        }
        return false;
    }

    // 6. Darth Maul Sudden Death check (2.10: < 2 balls on left flipper after grace)
    if sw_id == SW_LEFT_FLIPPER && in_multiball() && balls_in_play() < 2 && get_var("sudden_death_active") == 1 {
        set_var("sudden_death_active", 0);
        set_banner("SUDDEN DEATH! QUI-GON FALLS!", 2500);
        return false;
    }

    false
}

fn on_tick(dt_ms) {
    if is_mode_active("droid_hurry_up") {
        let v = get_var("droid_hurry_up_val") - (dt_ms * 56);
        if v < 100_000 {
            v = 100_000;
        }
        set_var("droid_hurry_up_val", v);
    }
    if is_mode_active("quick_multiball") && balls_in_play() < 2 && get_var("quick_mb_seen_2balls") == 1 {
        stop_mode("quick_multiball");
        set_var("quick_mb_seen_2balls", 0);
    }
    if is_mode_active("quick_multiball") && balls_in_play() >= 2 {
        set_var("quick_mb_seen_2balls", 1);
    }
}

fn on_mode_exit(name) {
    if name == "double_scoring" {
        set_score_multiplier(1);
        set_lamp(LAMP_EXTRA_BALL, false);
        set_banner("DOUBLE SCORING ENDED", 1500);
    }
    if name == "droid_hurry_up" {
        set_banner("DROID HURRY UP EXPIRED", 1500);
    }
    if name == "midnight_madness" {
        set_banner("MIDNIGHT MADNESS ENDED", 2000);
    }
}
