#[cfg(feature = "ansi")]
use std::{borrow::Cow, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Returns a hexadecimal string representing the colour.
    ///
    /// # Example
    ///
    /// ```
    /// TODO
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

#[cfg(feature = "css")]
impl From<Color> for css_colors::RGB {
    fn from(value: Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(value.0),
            g: css_colors::Ratio::from_u8(value.1),
            b: css_colors::Ratio::from_u8(value.2),
        }
    }
}

#[cfg(feature = "ansi")]
impl Color {
    pub fn ansi_paint<'a, I, S: 'a + ToOwned + ?Sized>(
        &self,
        input: I,
    ) -> ansi_term::ANSSiValue<'a, S>
    where
        I: Into<Cow<'a, S>>,
        <S as ToOwned>::Owned: Debug,
    {
        ansi_term::Color::RGB(self.0, self.1, self.2).paint(input)
    }
}

#[cfg(feature = "ratatui")]
impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        Self::Rgb(value.0, value.1, value.2)
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(colour: Color) -> Self {
        (colour.0, colour.1, colour.2)
    }
}
