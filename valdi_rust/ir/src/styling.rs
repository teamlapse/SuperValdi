//! Styling schema.
//!
//! ```
//! use valdi_rust_ir::styling::{Color, Display, Style};
//!
//! let style = Style::minimal(Color::rgba(0, 0, 0, 255));
//! assert_eq!(style.display, Display::Flex);
//! ```

use crate::extensions::PlatformExtension;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Background {
    None,
    Solid(Color),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Display {
    Flex,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassMetadata {
    pub class_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StylePlatformExtension {
    pub extension: PlatformExtension,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub background: Background,
    pub opacity: f32,
    pub border: Option<Border>,
    pub radius: Option<Radius>,
    pub shadow: Option<Shadow>,
    pub overflow: Overflow,
    pub visibility: Visibility,
    pub display: Display,
    pub class_metadata: Option<ClassMetadata>,
    pub platform_extension: Option<StylePlatformExtension>,
}

impl Style {
    pub const fn minimal(background: Color) -> Self {
        Self {
            background: Background::Solid(background),
            opacity: 1.0,
            border: None,
            radius: None,
            shadow: None,
            overflow: Overflow::Visible,
            visibility: Visibility::Visible,
            display: Display::Flex,
            class_metadata: None,
            platform_extension: None,
        }
    }
}
