use crate::components::ant_design::svg::helpers::svg_wrapper_props::SvgWrapperProps;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html};

#[function_component(SvgWrapper)]
pub fn svg_wrapper(props: &SvgWrapperProps) -> Html {
    let spin_class = match &props.spin {
        None => AttrValue::Static(""),
        Some(_) => AttrValue::Static("anticon-spin"),
    };
    let rotate_style = match &props.rotate {
        None => AttrValue::Static(""),
        Some(rotate) => {
            AttrValue::Owned(format!("transform: rotate({}deg);", rotate.get_degrees()))
        }
    };
    let view_box = match props.view_box.clone() {
        None => AttrValue::Static("64 64 896 896"),
        Some(view_box_handle) => view_box_handle,
    };
    let common_size = AttrValue::Static("1em");
    let width = match props.width.clone() {
        None => common_size.clone(),
        Some(width) => width,
    };
    let height = match props.height.clone() {
        None => common_size,
        Some(height) => height,
    };
    html! {
      <svg
        viewBox={view_box}
        focusable="false"
        class={spin_class.into_string()}
        width={width.clone()}
        height={height.clone()}
        fill={props.fill.clone().into_string_color()}
        aria-hidden="true"
        style={rotate_style}
      >
        {props.inner_html.clone()}
      </svg>
    }
}
