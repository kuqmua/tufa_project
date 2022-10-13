use crate::components::ant_design::svg::helpers::svg_component_trait::SvgComponent;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use svg_component::SvgComponent;
use yew::html;
use yew::virtual_dom::AttrValue;
use yew::Html;

#[derive(Debug, PartialEq, Clone, SvgComponent)]
pub enum SvgType {
    CheckCircle(SvgProps),
    CloseCircle(SvgProps),
    Close(SvgProps),
    Cloud(SvgProps),
    Copy(SvgProps),
    Dislike(SvgProps),
    Down(SvgProps),
    ExclamationCircle(SvgProps),
    Github(SvgProps),
    Heart(SvgProps),
    InfoCircle(SvgProps),
    Like(SvgProps),
    Loading(SvgProps),
    Login(SvgProps),
    Logout(SvgProps),
    Reddit(SvgProps),
    ShareAlt(SvgProps),
    Sync(SvgProps),
    Twitter(SvgProps),
    Up(SvgProps),
    User(SvgProps),
    Youtube(SvgProps),
}
