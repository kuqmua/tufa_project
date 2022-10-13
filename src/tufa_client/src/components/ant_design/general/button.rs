use crate::components::ant_design::general::icon::Icon;
// use crate::components::ant_design::svg::close::Close;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use crate::helpers::pseudo_css_wrapper::PseudoCssWrapper;
use crate::lazy_static::white_hsl::WHITE_HSL;
use web_sys::MouseEvent;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ButtonType {
    Primary,
    Ghost,
    Dashed,
    Danger,
    Link,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoadingProp {
    Bool(bool),
    Delay { delay: u32 },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Shape {
    Circle,
    Round,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Size {
    Small,
    Large,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub disabled: Option<()>, //or maybe explicit bool?
    pub ghost: Option<()>,
    pub href: Option<AttrValue>,
    pub html_type: Option<AttrValue>,
    pub icon: Option<Html>, //Icon Component
    pub loading: Option<LoadingProp>,
    pub shape: Option<Shape>,
    pub size: Option<Size>,
    pub target: Option<AttrValue>,
    pub button_type: Option<ButtonType>, //original "type"
    pub on_click: Option<Callback<MouseEvent>>,
    pub block: Option<()>,
    pub placeholder: Option<AttrValue>,

    pub style: Option<PseudoCssWrapper>, //tooltip required it
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    //todo: button group
    let block_class = match &props.block {
        None => AttrValue::Static(""),
        Some(_) => AttrValue::Static("ant-btn-block"),
    };
    let loading_class = match &props.loading {
        None => AttrValue::Static(""),
        Some(_) => AttrValue::Static("ant-btn-loading"),
    };
    let ghost_class = match &props.ghost {
        None => AttrValue::Static(""),
        Some(_) => AttrValue::Static("ant-btn-background-ghost"),
    };
    let button_only_class = match &props.placeholder {
        None => match &props.icon {
            None => AttrValue::Static(""),
            Some(_) => AttrValue::Static("ant-btn-icon-only"),
        },
        Some(_) => AttrValue::Static(""),
    };
    let size_class = match &props.size {
        None => AttrValue::Static(""),
        Some(size) => match size {
            Size::Small => AttrValue::Static("ant-btn-sm"),
            Size::Large => AttrValue::Static("ant-btn-lg"),
        },
    };
    let shape_class = match &props.shape {
        None => AttrValue::Static(""),
        Some(shape) => match shape {
            Shape::Circle => AttrValue::Static("ant-btn-circle"),
            Shape::Round => AttrValue::Static("ant-btn-round"), //todo
        },
    };
    let button_type_class = match &props.button_type {
        None => AttrValue::Static(""),
        Some(button_type) => match button_type {
            ButtonType::Primary => AttrValue::Static("ant-btn-primary"),
            ButtonType::Ghost => AttrValue::Static("ant-btn-ghost"),
            ButtonType::Dashed => AttrValue::Static("ant-btn-dashed"),
            ButtonType::Danger => AttrValue::Static("ant-btn-danger"),
            ButtonType::Link => AttrValue::Static("ant-btn-link"),
        },
    };
    let classes = format!(
        "ant-btn {} {} {} {} {} {} {}",
        button_type_class,
        shape_class,
        button_only_class,
        size_class,
        ghost_class,
        loading_class,
        block_class
    );
    let placeholder = match &props.placeholder {
        None => html!(""),
        Some(placeholder) => html! {
          <span>
            {placeholder}
          </span>
        },
    };
    let is_button_disabled = props.disabled.is_some();
    let icon = match props.loading {
        None => match &props.icon {
            None => html! {},
            Some(icon) => html! {{icon.clone()}},
        },
        Some(_) => {
            html! {
              <Icon
                svg_type={
                  SvgType::Loading(
                    SvgProps{
                      height: None,
                      width: None,
                      fill: Some(FillWith::Hsl(WHITE_HSL.clone())),
                      spin: None,
                      rotate: None,
                      theme: None,
                    }
                  )
                }
                additional_class={String::from("anticon-close")}
              />
            }
        }
    };
    html! {
      <button disabled={is_button_disabled} type="button" class={classes}>
        {icon}
        {placeholder}
      </button>
    }
}
