use crate::components::ant_design::svg::helpers::fill_with::FillWith;

pub fn get_color(option_color: Option<FillWith>) -> FillWith {
    match option_color {
        None => FillWith::Hsl(Hsl::new(0.0, 100.0, 100.0, Some(1.0))), //as default
        Some(fill) => fill,
    }
}
