//! Describe theme available in the game

use ggez::graphics::Color;

use crate::{NUM_COLORS, NUM_PIECES};

pub const THEME_DUST: Theme = Theme {
    board_color: [
        Color::new(0.7969, 0.7148, 0.6797, 1.0),
        Color::new(0.4375, 0.3984, 0.4648, 1.0),
    ],
    piece_path: [
        [
            "/images/pieces/white_pawn.png",
            "/images/pieces/white_knight.png",
            "/images/pieces/white_bishop.png",
            "/images/pieces/white_rook.png",
            "/images/pieces/white_queen.png",
            "/images/pieces/white_king.png",
        ],
        [
            "/images/pieces/black_pawn.png",
            "/images/pieces/black_knight.png",
            "/images/pieces/black_bishop.png",
            "/images/pieces/black_rook.png",
            "/images/pieces/black_queen.png",
            "/images/pieces/black_king.png",
        ],
    ],
    valid_moves_color: Some(Color::new(0.25, 0.75, 0.25, 0.5)),
    piece_pinned_color: Some(Color::new(0.75, 0.25, 0.25, 0.5)),
    piece_pinned_path: Some("/images/pin.png"),
    background_color: Color::new(0.0, 0.0, 0.0, 1.0),
    font_path: "/fonts/LiberationMono-Regular.ttf",
    font_scale: 15.0,
};

pub const THEME_CORAL: Theme = Theme {
    board_color: [
        Color::new(177.0 / 256.0, 228.0 / 256.0, 185.0 / 256.0, 1.0),
        Color::new(112.0 / 256.0, 162.0 / 256.0, 163.0 / 256.0, 1.0),
    ],
    ..THEME_DUST
};

pub const THEME_MARINE: Theme = Theme {
    board_color: [
        Color::new(157.0 / 256.0, 172.0 / 256.0, 255.0 / 256.0, 1.0),
        Color::new(111.0 / 256.0, 115.0 / 256.0, 210.0 / 256.0, 1.0),
    ],
    ..THEME_DUST
};

pub const THEME_WHEAT: Theme = Theme {
    board_color: [
        Color::new(234.0 / 256.0, 240.0 / 256.0, 206.0 / 256.0, 1.0),
        Color::new(187.0 / 256.0, 190.0 / 256.0, 100.0 / 256.0, 1.0),
    ],
    ..THEME_DUST
};

pub const THEME_EMERALD: Theme = Theme {
    board_color: [
        Color::new(173.0 / 256.0, 189.0 / 256.0, 143.0 / 256.0, 1.0),
        Color::new(111.0 / 256.0, 143.0 / 256.0, 114.0 / 256.0, 1.0),
    ],
    ..THEME_DUST
};

pub const THEME_SANDCASTLE: Theme = Theme {
    board_color: [
        Color::new(227.0 / 256.0, 193.0 / 256.0, 111.0 / 256.0, 1.0),
        Color::new(184.0 / 256.0, 139.0 / 256.0, 74.0 / 256.0, 1.0),
    ],
    ..THEME_DUST
};

/// Describe the theme of the chess game (GUI).
///
/// RootPath is `resources/` (changed by [`ggez::ContextBuilder::add_resource_path`]).
///
/// # Examples
///
/// ```
/// use chess::{Theme, THEME_DUST};
///
/// const THEME:Theme = Theme {
///     font_path: "/fonts/font.ttf", // located in resources/fonts/font.ttf
///     ..THEME_DUST
/// };
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Theme {
    // Board
    pub board_color: [Color; NUM_COLORS],
    pub piece_path: [[&'static str; NUM_PIECES]; NUM_COLORS],
    pub valid_moves_color: Option<Color>,
    pub piece_pinned_color: Option<Color>,
    pub piece_pinned_path: Option<&'static str>,
    pub background_color: Color,
    pub font_path: &'static str,
    pub font_scale: f32,
}

impl Default for Theme {
    fn default() -> Self {
        THEME_DUST
    }
}
