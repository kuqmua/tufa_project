use colorsys::Hsl;
use yew::virtual_dom::AttrValue;

#[derive(Debug, PartialEq, Clone)]
pub enum FillWith {
    Hsl(Hsl),
    CurrentColor,
}

impl FillWith {
    pub fn into_string_color(&self) -> AttrValue {
        match self {
            FillWith::Hsl(hsl) => AttrValue::Owned(hsl.to_css_string()),
            FillWith::CurrentColor => AttrValue::Static("currentColor"),
        }
    }
}
