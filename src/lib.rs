#[cfg(feature = "css")]
pub use css_colors;

mod colour;
pub use colour::Colour;

mod flavour_colours;
pub use flavour_colours::FlavourColours;

mod flavour;
pub use flavour::Flavour;

#[cfg(test)]
mod tests {
    use super::Flavour;

    #[test]
    fn test_flavours_iter() {
        let mut flavours = Flavour::all().into_iter();
        assert_eq!(flavours.next(), Some(Flavour::Latte));
        assert_eq!(flavours.next(), Some(Flavour::Frappe));
        assert_eq!(flavours.next(), Some(Flavour::Macchiato));
        assert_eq!(flavours.next(), Some(Flavour::Mocha));
        assert_eq!(flavours.next(), None);
    }

    #[test]
    fn test_colours_iter() {
        for flavour in Flavour::all() {
            let colours = flavour.colours();
            let mut colours_iter = colours.into_iter();
            assert_eq!(colours_iter.next(), Some(colours.rosewater));
            assert_eq!(colours_iter.next(), Some(colours.flamingo));
            assert_eq!(colours_iter.next(), Some(colours.pink));
            assert_eq!(colours_iter.next(), Some(colours.mauve));
            assert_eq!(colours_iter.next(), Some(colours.red));
            assert_eq!(colours_iter.next(), Some(colours.maroon));
            assert_eq!(colours_iter.next(), Some(colours.peach));
            assert_eq!(colours_iter.next(), Some(colours.yellow));
            assert_eq!(colours_iter.next(), Some(colours.green));
            assert_eq!(colours_iter.next(), Some(colours.teal));
            assert_eq!(colours_iter.next(), Some(colours.sky));
            assert_eq!(colours_iter.next(), Some(colours.sapphire));
            assert_eq!(colours_iter.next(), Some(colours.blue));
            assert_eq!(colours_iter.next(), Some(colours.lavender));
            assert_eq!(colours_iter.next(), Some(colours.text));
            assert_eq!(colours_iter.next(), Some(colours.subtext1));
            assert_eq!(colours_iter.next(), Some(colours.subtext0));
            assert_eq!(colours_iter.next(), Some(colours.overlay2));
            assert_eq!(colours_iter.next(), Some(colours.overlay1));
            assert_eq!(colours_iter.next(), Some(colours.overlay0));
            assert_eq!(colours_iter.next(), Some(colours.surface2));
            assert_eq!(colours_iter.next(), Some(colours.surface1));
            assert_eq!(colours_iter.next(), Some(colours.surface0));
            assert_eq!(colours_iter.next(), Some(colours.base));
            assert_eq!(colours_iter.next(), Some(colours.mantle));
            assert_eq!(colours_iter.next(), Some(colours.crust));
        }
    }
}
