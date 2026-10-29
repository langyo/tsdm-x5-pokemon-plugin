//! 宠物精灵图地址 — 普通/异色（闪光）走不同目录。
//!
//! 目录约定（X3 时代图床素材，大小写与 CDN 一致）：
//! - 正面：普通 `pm`，异色 `pmsg`
//! - 背面：普通 `pmb`，异色 `pmsgb`
//!
//! CDN 缺图时由 plugin/game.inc.php 的全局 onerror 回退到本地
//! `plugin/images/{dir}/{species}.png`。

use crate::components::layout::IMG_PATH_REMOTE;

/// 正面精灵图地址（普通 `pm` / 异色 `pmsg`）。
pub fn front_sprite_url(species_id: u64, is_shiny: bool) -> String {
    let dir = if is_shiny { "pmsg" } else { "pm" };
    format!("{IMG_PATH_REMOTE}/{dir}/{species_id}.gif")
}

/// 背面精灵图地址（普通 `pmb` / 异色 `pmsgb`）。
pub fn back_sprite_url(species_id: u64, is_shiny: bool) -> String {
    let dir = if is_shiny { "pmsgb" } else { "pmb" };
    format!("{IMG_PATH_REMOTE}/{dir}/{species_id}.gif")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_front_sprite_uses_pm_directory() {
        assert_eq!(
            front_sprite_url(384, false),
            "https://img.tsdm39.com/Pokemon/pm/384.gif"
        );
    }

    #[test]
    fn shiny_front_sprite_uses_pmsg_directory() {
        assert_eq!(
            front_sprite_url(384, true),
            "https://img.tsdm39.com/Pokemon/pmsg/384.gif"
        );
    }

    #[test]
    fn normal_back_sprite_uses_pmb_directory() {
        assert_eq!(
            back_sprite_url(384, false),
            "https://img.tsdm39.com/Pokemon/pmb/384.gif"
        );
    }

    #[test]
    fn shiny_back_sprite_uses_pmsgb_directory() {
        assert_eq!(
            back_sprite_url(384, true),
            "https://img.tsdm39.com/Pokemon/pmsgb/384.gif"
        );
    }
}
