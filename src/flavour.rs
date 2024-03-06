use crate::{Colour, FlavourColours};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flavour {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

macro_rules! impl_colour_method {
    ($x: ident) => {
        pub const fn $x(self) -> $crate::Colour {
            self.colours().$x
        }
    };
    ($x:ident, $($y:ident), + $(,)?) => {
        pub const fn $x(self) -> $crate::Colour {
            self.colours().$x
        }

        impl_colour_method!($($y),+);
    };
}

impl Flavour {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Latte => "latte",
            Self::Frappe => "frappe",
            Self::Macchiato => "macchiato",
            Self::Mocha => "mocha",
        }
    }
}
