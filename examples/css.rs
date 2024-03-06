use css_colors::{percent, Color};
use rc_color::Flavour;

fn main() {
    let teal = Flavour::Latte.teal();
    let rgb: css_colors::RGB = teal.into();

    println!("RGB: {}", rgb.to_css());

    let hsl = rgb.to_hsl();
    println!("HSL: {}", hsl.to_css());

    let lighter = hsl.lighten(percent(20));
    println!("20% lighter: {lighter}");
}
