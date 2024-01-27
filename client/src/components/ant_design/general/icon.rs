use crate::components::ant_design::svg::helpers::svg_component_trait::SvgComponent;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub svg_type: SvgType,
    pub style: Option<AttrValue>, //CSSProperties
    pub additional_class: Option<AttrValue>,
    // pub theme: Option<Theme>,//explicit svg styles
    // moved into svg
    // pub component: ComponentType<CustomIconComponentProps>//todo
    // pub two_tone_color: //todo
    //pub spin: Option<()> // moved into svg
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let additional_class = match props.additional_class.clone() {
        None => AttrValue::Static(""),
        Some(class) => class,
    };
    let class = format!("anticon {}", additional_class);
    html! {
        <i class={class}>//todo: aria-label="icon: home" (example)
          {props.svg_type.get_html()}
        </i>
    }
}
