//! 24-bit `TokyoNight` Storm palette and glyphs for `lida next`.

use super::span::Rgb;

pub const ID: Rgb = (0x7a, 0xa2, 0xf7);
pub const CHIP: Rgb = (0x9e, 0xce, 0x6a);
pub const CHIP_TEXT: Rgb = (0x1d, 0x20, 0x2f);
pub const PRIORITY: Rgb = (0xe0, 0xaf, 0x68);
pub const OVERDUE: Rgb = (0xf7, 0x76, 0x8e);
pub const SOON: Rgb = (0xc0, 0xca, 0xf5);
pub const LATER: Rgb = (0xa9, 0xb1, 0xd6);
pub const MUTED: Rgb = (0x73, 0x7a, 0xa2);
pub const TAGS: Rgb = (0x03, 0xff, 0xd2);
pub const TEXT: Rgb = (0xc0, 0xca, 0xf5);
pub const HEADER_BG: Rgb = (0x16, 0x1b, 0x22);
pub const BACKGROUND: Rgb = (0x1d, 0x21, 0x2f);

pub const PROJECTS: [Rgb; 8] = [
    (0x12, 0xae, 0xdb),
    (0xd7, 0x8b, 0x3d),
    (0x36, 0xba, 0x6d),
    (0xd6, 0xda, 0x4d),
    (0xaa, 0x86, 0xf5),
    (0xd9, 0x58, 0xc0),
    (0x4b, 0xe9, 0xfc),
    (0xfd, 0xb6, 0xf0),
];

pub const DISC_H: char = '\u{F0C01}';
pub const DISC_M: char = '\u{F0C10}';
pub const DISC_L: char = '\u{F0C0D}';
pub const CHIP_LEFT: char = '\u{2595}';
pub const CHIP_RIGHT: char = '\u{258F}';

/// Stable FNV-1a 32-bit hash mapping a top-level project name to a palette colour.
#[must_use]
pub fn project_colour(top: &str) -> Rgb {
    let mut hash: u32 = 0x811c_9dc5;
    for &byte in top.as_bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    PROJECTS[(hash as usize) % PROJECTS.len()]
}

/// Mixes `c` at 70% over `BACKGROUND` for subproject suffixes.
#[must_use]
pub fn dimmed(c: Rgb) -> Rgb {
    let mix = |fg: u8, bg: u8| -> u8 {
        let weighted = (u16::from(fg) * 70 + u16::from(bg) * 30 + 50) / 100;
        u8::try_from(weighted).unwrap_or(u8::MAX)
    };
    (mix(c.0, BACKGROUND.0), mix(c.1, BACKGROUND.1), mix(c.2, BACKGROUND.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_matches_spec_hex_values() {
        assert_eq!(ID, (0x7a, 0xa2, 0xf7));
        assert_eq!(CHIP, (0x9e, 0xce, 0x6a));
        assert_eq!(CHIP_TEXT, (0x1d, 0x20, 0x2f));
        assert_eq!(PRIORITY, (0xe0, 0xaf, 0x68));
        assert_eq!(OVERDUE, (0xf7, 0x76, 0x8e));
        assert_eq!(SOON, (0xc0, 0xca, 0xf5));
        assert_eq!(LATER, (0xa9, 0xb1, 0xd6));
        assert_eq!(MUTED, (0x73, 0x7a, 0xa2));
        assert_eq!(TAGS, (0x03, 0xff, 0xd2));
        assert_eq!(TEXT, (0xc0, 0xca, 0xf5));
        assert_eq!(HEADER_BG, (0x16, 0x1b, 0x22));
        assert_eq!(BACKGROUND, (0x1d, 0x21, 0x2f));
        let expected_projects = [
            0x12_ae_db_u32,
            0xd7_8b_3d,
            0x36_ba_6d,
            0xd6_da_4d,
            0xaa_86_f5,
            0xd9_58_c0,
            0x4b_e9_fc,
            0xfd_b6_f0,
        ]
        .map(|h| (((h >> 16) & 0xff) as u8, ((h >> 8) & 0xff) as u8, (h & 0xff) as u8));
        assert_eq!(PROJECTS, expected_projects);
        assert_eq!(DISC_H, '\u{F0C01}');
        assert_eq!(DISC_M, '\u{F0C10}');
        assert_eq!(DISC_L, '\u{F0C0D}');
        assert_eq!(CHIP_LEFT, '\u{2595}');
        assert_eq!(CHIP_RIGHT, '\u{258F}');
    }

    #[test]
    fn project_colour_is_stable_and_in_palette() {
        let c = project_colour("lida");
        assert_eq!(c, project_colour("lida"));
        assert!(PROJECTS.contains(&c));
    }

    #[test]
    fn dimmed_mixes_seventy_percent_over_background() {
        assert_eq!(dimmed((0x12, 0xae, 0xdb)), (0x15, 0x84, 0xa7));
    }
}
