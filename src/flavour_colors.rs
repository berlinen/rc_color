use crossterm::style::Color;

use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlavourColors {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}
impl IntoIterator for FlavourColors {
    type Item = Color;
    type IntoIter = std::array::IntoIter<Self::Item, 26>;

    /// Returns an iterator over the colours in the flavour.
    fn into_iter(self) -> Self::IntoIter {
        [
            self.rosewater,
            self.flamingo,
            self.pink,
            self.mauve,
            self.red,
            self.maroon,
            self.peach,
            self.yellow,
            self.green,
            self.teal,
            self.sky,
            self.sapphire,
            self.blue,
            self.lavender,
            self.text,
            self.subtext1,
            self.subtext0,
            self.overlay2,
            self.overlay1,
            self.overlay0,
            self.surface2,
            self.surface1,
            self.surface0,
            self.base,
            self.mantle,
            self.crust,
        ]
        .into_iter()
    }
}

impl FlavourColors {
    /// Returns an iterator over (name, colour) pairs in the flavour.
    /// This can be useful for constructing a map of the colours:
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use catppuccin::Flavour;
    ///
    /// let colours: HashMap<_, _> = Flavour::Mocha.colours().into_fields_iter().collect();
    /// println!("Red: {}", colours["red"].hex());
    /// ```
    pub fn into_fields_iter(self) -> std::array::IntoIter<(&'static str, Color), 26> {
        [
            ("rosewater", self.rosewater),
            ("flamingo", self.flamingo),
            ("pink", self.pink),
            ("mauve", self.mauve),
            ("red", self.red),
            ("maroon", self.maroon),
            ("peach", self.peach),
            ("yellow", self.yellow),
            ("green", self.green),
            ("teal", self.teal),
            ("sky", self.sky),
            ("sapphire", self.sapphire),
            ("blue", self.blue),
            ("lavender", self.lavender),
            ("text", self.text),
            ("subtext1", self.subtext1),
            ("subtext0", self.subtext0),
            ("overlay2", self.overlay2),
            ("overlay1", self.overlay1),
            ("overlay0", self.overlay0),
            ("surface2", self.surface2),
            ("surface1", self.surface1),
            ("surface0", self.surface0),
            ("base", self.base),
            ("mantle", self.mantle),
            ("crust", self.crust),
        ]
        .into_iter()
    }
}
