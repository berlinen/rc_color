#[cfg(feature = "css")]
pub use css_colors;

mod colour;
pub use colour::Colour;

mod flavour_colours;
pub use flavour_colours::FlavourColours;

mod flavour;
pub use flavour::Flavour;
