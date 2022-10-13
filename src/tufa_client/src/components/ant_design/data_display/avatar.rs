use crate::components::ant_design::svg::helpers::svg_component_trait::SvgComponent;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html, Callback, Properties};

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarShape {
    Circle,
    Square,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarSizeType {
    Large,
    Small,
    Default,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarSize {
    Number(u16),
    Type(AvatarSizeType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AvatarContent {
    Icon(SvgType),
    Image(AvatarImage),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AvatarImage {
    pub src: AttrValue,
    // pub src_set	a list of sources to use for different screen resolutions	string	-	3.11.0 //no examples for it yet in antd docs
    pub alt: AttrValue,
    pub on_error: Option<Callback<yew::Event>>,
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub content: Option<AvatarContent>,
    pub shape: Option<AvatarShape>,
    pub size: Option<AvatarSize>,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let size_style: AttrValue;
    let size_class: AttrValue;
    match props.size.clone() {
        None => {
            size_style = AttrValue::Static("");
            size_class = AttrValue::Static("");
        }
        Some(size_type) => match size_type {
            AvatarSize::Number(size) => {
                size_style = AttrValue::Owned(format!(
                    "width: {}px; height: {}px; line-height: {}px; font-size: 18px;",
                    size, size, size
                ));
                size_class = AttrValue::Static("");
            }
            AvatarSize::Type(size_type) => match size_type {
                AvatarSizeType::Large => {
                    size_style = AttrValue::Static("");
                    size_class = AttrValue::Static("ant-avatar-lg");
                }
                AvatarSizeType::Small => {
                    size_style = AttrValue::Static("");
                    size_class = AttrValue::Static("ant-avatar-sm");
                }
                AvatarSizeType::Default => {
                    size_style = AttrValue::Static("");
                    size_class = AttrValue::Static("");
                }
            },
        },
    };
    let shape_class = match props.shape.clone() {
        None => AttrValue::Static("ant-avatar-circle"),
        Some(shape_type) => match shape_type {
            AvatarShape::Circle => AttrValue::Static("ant-avatar-circle"),
            AvatarShape::Square => AttrValue::Static("ant-avatar-square"),
        },
    };
    let content_class = match props.content.clone() {
        None => AttrValue::Static("ant-avatar-icon"),
        Some(avatar_content) => match avatar_content {
            AvatarContent::Icon(_) => AttrValue::Static("ant-avatar-icon"),
            AvatarContent::Image(_) => AttrValue::Static("ant-avatar-image"),
        },
    };
    let inner_content = match props.content.clone() {
        None => {
            let svg_type = SvgType::User(SvgProps {
                height: None,
                width: None,
                fill: None,
                spin: None,
                rotate: None,
                theme: None,
            });
            let class = format!("anticon {}", svg_type.get_class());
            html! {
                <i aria-label="icon: user" class={class}>
                  {svg_type.get_html()}
                </i>
            }
        }
        Some(content_type) => match content_type {
            AvatarContent::Icon(svg_type) => {
                let class = format!("anticon {}", svg_type.get_class());
                html! {
                    <i aria-label="icon: user" class={class}>
                      {svg_type.get_html()}
                    </i>
                }
            }
            AvatarContent::Image(avatar_image) => html! {
                <img src={avatar_image.src} alt={avatar_image.alt} onerror={avatar_image.on_error} />
            },
        },
    };
    let style = format!("{}", size_style);
    let class = format!(
        "ant-avatar {} {} {}",
        shape_class, size_class, content_class
    );
    html! {
        <span class={class} style={style}>
          {inner_content}
        </span>
    }
}
