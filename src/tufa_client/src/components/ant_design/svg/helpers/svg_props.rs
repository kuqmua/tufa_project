use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::theme::Theme;
use crate::helpers::rotate::Rotate;
use yew::virtual_dom::AttrValue;
use yew::Properties;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SvgProps {
    pub height: Option<AttrValue>,
    pub width: Option<AttrValue>,
    pub fill: Option<FillWith>,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub theme: Option<Theme>,
}
