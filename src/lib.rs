#[cfg(feature = "css")]
pub use css_colors;

mod color;
pub use color::Color;

mod flavour_colors;
pub use flavour_colors::FlavourColors;
