use crate::components::ant_design::general::icon::Icon;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_type::SvgType;
use crate::components::ant_design::svg::helpers::theme::Theme;
use colorsys::Hsl;
use web_sys::MouseEvent;
use yew::virtual_dom::AttrValue;
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Debug, PartialEq, Clone)]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Error,
}

impl AlertType {
    pub fn get_class(&self) -> AttrValue {
        match self {
            AlertType::Success => AttrValue::Static("ant-alert-success"),
            AlertType::Info => AttrValue::Static("ant-alert-info"),
            AlertType::Warning => AttrValue::Static("ant-alert-warning"),
            AlertType::Error => AttrValue::Static("ant-alert-error"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    // pub after_close: Option<Callback<()>>,//animation not done yet, todo
    pub banner: Option<()>,
    pub closable: Option<()>,
    pub close_text: Option<AttrValue>,  //Html
    pub description: Option<AttrValue>, //Html
    pub icon: Option<SvgType>,
    pub message: Option<AttrValue>,
    pub show_icon: Option<()>,
    pub type_handle: Option<AlertType>,
    pub on_close: Option<Callback<()>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlertChangingStyleState {
    Opened,
    Closing,
    Closed,
}

impl AlertChangingStyleState {
    pub fn get_class(&self) -> AttrValue {
        match *self {
            AlertChangingStyleState::Opened => AttrValue::Static("ant-alert-closable"),
            AlertChangingStyleState::Closing => AttrValue::Static("ant-alert-closing ant-alert-no-icon ant-alert-closable ant-alert-slide-up-leave ant-alert-slide-up-leave-active"),
            AlertChangingStyleState::Closed => AttrValue::Static(""),
        }
    }
    pub fn get_value(&self) -> AlertChangingStyle {
        match *self {
            AlertChangingStyleState::Opened => AlertChangingStyle {
                should_render: true,
                height: AttrValue::Static("100%"),
                opacity: AttrValue::Static(""),
            },
            AlertChangingStyleState::Closing => AlertChangingStyle {
                should_render: true,
                height: AttrValue::Static("0%"),
                opacity: AttrValue::Static(""),
            },
            AlertChangingStyleState::Closed => AlertChangingStyle {
                should_render: false,
                height: AttrValue::Static("0px"),
                opacity: AttrValue::Static("0.5"),
            },
        }
    }
    pub fn get_style(&self) -> String {
        let value = self.get_value();
        format!("transition: transform 0.3s; {}", value.height)
    }
}

#[derive(Debug, PartialEq)]
pub struct AlertChangingStyle {
    pub should_render: bool,
    pub height: AttrValue,
    pub opacity: AttrValue,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    // let style = use_state(|| String::from(""));
    // let cloned_style = style.clone();
    let alert_changing_style = use_state(|| AlertChangingStyleState::Opened);
    let alert_changing_style_second_clone = alert_changing_style.clone();
    let handle_close = {
        let on_close_clone = props.on_close.clone();
        // let after_close_clone = props.after_close.clone();
        // let style_clone = style.clone();
        Callback::<MouseEvent>::from(move |e: MouseEvent| {
            let alert_changing_style_cloned = alert_changing_style.clone();
            e.prevent_default();
            // // const dom = ReactDOM.findDOMNode(this) as HTMLElement;
            // // dom.style.height = "${dom.offsetHeight}px";
            // style_clone.set(String::from("${dom.offsetHeight}px"));
            // // Magic code
            // // 重复一次后才能正确设置 height
            // // dom.style.height = `${dom.offsetHeight}px`;
            // style_clone.set(String::from("${dom.offsetHeight}px"));
            if let Some(on_close) = on_close_clone.clone() {
                on_close.emit(());
            };
            alert_changing_style_cloned.set(AlertChangingStyleState::Closed);
            //AlertChangingStyleState::Closing
            // let after_close_clone_clone = after_close_clone.clone();
            // gloo::timers::callback::Timeout::new(300, move || {
            //     //0.3second from antd.css
            //     // let after_close_clone_clone_clone = after_close_clone_clone.clone();
            //     // if let Some(after_close) = after_close_clone_clone_clone {
            //     //     after_close.emit(());
            //     // };
            //     let alert_changing_style_cloned_cloned = alert_changing_style_cloned.clone();
            //     alert_changing_style_cloned_cloned.set(AlertChangingStyleState::Closed);
            // })
            // .forget();
        })
    };
    let message = match props.message.clone() {
        None => AttrValue::Static(""),
        Some(msg) => msg,
    };
    let description = match props.description.clone() {
        None => AttrValue::Static(""),
        Some(desc) => desc,
    };
    let type_handle = match props.type_handle.clone() {
        None => AlertType::Info,
        Some(alert_type) => alert_type,
    };
    let description_class = match props.description.clone() {
        None => AttrValue::Static(""),
        Some(_) => AttrValue::Static("ant-alert-with-description"),
    };
    let show_icon_class = match props.show_icon {
        None => AttrValue::Static("ant-alert-no-icon"),
        Some(_) => AttrValue::Static(""),
    };
    let class = format!(
        "ant-alert {} {} {}", // {}
        type_handle.get_class(),
        description_class,
        show_icon_class,
        // alert_changing_style_second_clone.get_class(),
    );
    let close_button = match props.closable {
        None => match props.close_text.clone() {
            None => html! {},
            Some(text) => html! {
                <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">
                  <span class="ant-alert-close-text">{text}</span></button>
            },
        },
        Some(_) => match props.close_text.clone() {
            None => {
                html! {
                  <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
                    <Icon
                      svg_type={
                        SvgType::Close(
                          SvgProps{
                            height: None,
                            width: None,
                            fill: Some(FillWith::Hsl(Hsl::new(0.0, 100.0, 0.0, Some(1.0)))),
                            spin: None,
                            rotate: None,
                            theme: None,
                          }
                        )
                      }
                      additional_class={String::from("anticon-close")}
                    />
                  </button>
                }
            }
            Some(text) => html! {
                <button onclick={handle_close} type="button" class="ant-alert-close-icon" tabindex="0">// onclick={on_close}
                  <span class="ant-alert-close-text">{text}</span></button>
            },
        },
    };
    let icon = match props.show_icon {
        None => html! {},
        Some(_) => {
            let theme = match props.description.clone() {
                Some(_) => Theme::Outlined,
                None => Theme::Filled,
            };
            let svg_props = SvgProps {
                height: None,
                width: None,
                fill: Some(FillWith::CurrentColor),
                spin: None,
                rotate: None,
                theme: Some(theme),
            };
            match type_handle {
                AlertType::Success => {
                    html! {
                      <Icon
                        svg_type={SvgType::CheckCircle(svg_props)}
                        additional_class={String::from("anticon-check-circle ant-alert-icon")}
                      />
                    }
                }
                AlertType::Info => {
                    html! {
                      <Icon
                        svg_type={SvgType::InfoCircle(svg_props)}
                        additional_class={String::from("anticon-info-circle ant-alert-icon")}
                      />
                    }
                }
                AlertType::Warning => {
                    html! {
                      <Icon
                        svg_type={SvgType::ExclamationCircle(svg_props)}
                        additional_class={String::from("anticon-exclamation-circle ant-alert-icon")}
                      />
                    }
                }
                AlertType::Error => {
                    html! {
                      <Icon
                        svg_type={SvgType::CloseCircle(svg_props)}
                        additional_class={String::from("anticon-close-circle ant-alert-icon")}
                      />
                    }
                }
            }
        }
    };
    let should_render = alert_changing_style_second_clone.get_value().should_render;
    // let style_handle = &*cloned_style;
    html! {
      <>
        if should_render {
          <div data-show="true" class={class}>// style={style_handle.clone()}
            {icon}
            <span class="ant-alert-message">
              {message}
            </span>
            <span class="ant-alert-description">
              {description}
            </span>
              {close_button}
          </div>
        }
      </>
    }
}
