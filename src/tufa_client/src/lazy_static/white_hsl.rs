use colorsys::Hsl;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WHITE_HSL: Hsl = Hsl::new(0.0, 100.0, 100.0, Some(1.0));
}
