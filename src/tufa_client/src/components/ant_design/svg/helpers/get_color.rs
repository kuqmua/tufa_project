use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::lazy_static::white_hsl::WHITE_HSL;

pub fn get_color(option_color: Option<FillWith>) -> FillWith {
    match option_color {
        None => FillWith::Hsl(WHITE_HSL.clone()), //as default
        Some(fill) => fill,
    }
}
