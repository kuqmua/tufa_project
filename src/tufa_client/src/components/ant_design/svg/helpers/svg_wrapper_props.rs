use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::helpers::rotate::Rotate;
use yew::virtual_dom::AttrValue;
use yew::{Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SvgWrapperProps {
    pub height: Option<AttrValue>,
    pub width: Option<AttrValue>,
    pub fill: FillWith,
    pub spin: Option<()>,
    pub rotate: Option<Rotate>,
    pub view_box: Option<AttrValue>,
    pub inner_html: Html,
}
