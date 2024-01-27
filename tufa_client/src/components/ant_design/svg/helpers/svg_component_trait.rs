use yew::virtual_dom::AttrValue;
use yew::Html;

pub trait SvgComponent {
    fn get_html(&self) -> Html;
    fn get_class(&self) -> AttrValue;
}
