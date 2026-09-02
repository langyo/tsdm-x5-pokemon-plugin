pub fn hp_percent(hp: i64, max_hp: i64) -> u32 {
    if max_hp <= 0 {
        return 0;
    }
    ((hp.max(0) as f64) / (max_hp as f64) * 100.0) as u32
}

pub fn hp_class(hp: i64, max_hp: i64) -> &'static str {
    if max_hp <= 0 || hp <= 0 {
        return "hp-low";
    }
    let pct = (hp as f64) / (max_hp as f64);
    if pct > 0.5 {
        "hp-high"
    } else if pct > 0.2 {
        "hp-mid"
    } else {
        "hp-low"
    }
}

pub fn hp_class_storage(hp: i64, max_hp: i64) -> &'static str {
    if max_hp <= 0 || hp <= 0 {
        return "hp-bar-critical";
    }
    let pct = (hp as f64) / (max_hp as f64);
    if pct > 0.5 {
        "hp-bar-high"
    } else if pct > 0.2 {
        "hp-bar-mid"
    } else {
        "hp-bar-low"
    }
}

pub fn hp_health_status(hp: i64, max_hp: i64) -> &'static str {
    if max_hp <= 0 || hp <= 0 {
        "critical"
    } else {
        let pct = (hp as f64) / (max_hp as f64);
        if pct <= 0.2 {
            "danger"
        } else if pct <= 0.5 {
            "warning"
        } else {
            "healthy"
        }
    }
}

pub fn gender_symbol(gender: u8) -> &'static str {
    match gender {
        1 => "♂",
        2 => "♀",
        _ => "",
    }
}

pub fn gender_class(gender: u8) -> &'static str {
    match gender {
        1 => "male",
        2 => "female",
        _ => "",
    }
}

pub fn is_negative_state(state: u8) -> bool {
    matches!(state, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15 | 20 | 21 | 22)
}

pub fn is_weak_state(state: u8) -> bool {
    matches!(state, 20..=22)
}
