use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::Html;
use yew::Properties;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct TransBtnProps {
    pub class_names: Vec<String>,
    pub customize_icon: Option<Html>,                //todo types
    pub on_mouse_down: Option<Callback<MouseEvent>>, //React.MouseEventHandler<HTMLSpanElement>
    pub on_click: Option<Callback<MouseEvent>>,      //React.MouseEventHandler<HTMLSpanElement>
    pub children: Children,
}

#[function_component(TransBtn)]
pub fn trans_btn(props: &TransBtnProps) -> Html {
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: MouseEvent| {}),
        Some(okd) => okd,
    };
    let on_mouse_down = match props.on_mouse_down.clone() {
        None => Callback::from(|e: MouseEvent| {
            e.prevent_default();
        }),
        Some(okd) => Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            okd.emit(e);
        }),
    };
    let content = match props.customize_icon.clone() {
        None => {
            let icon_classes = props
                .class_names
                .clone()
                .iter()
                .map(|cls| format!("{}-icon", cls))
                .fold(String::from(""), |mut acc, elem| {
                    acc.push_str(&elem);
                    acc
                });
            html! {
              <span class={icon_classes}>
                {props.children.clone()}
              </span>
            }
        }
        Some(ci) => ci,
    };
    let span_class = props
        .class_names
        .clone()
        .iter()
        .fold(String::from(""), |mut acc, elem| {
            acc.push_str(elem);
            acc
        });
    html! {
      <span
        class={span_class}
        onmousedown={on_mouse_down.clone()}
        style={"user-select: none; -webkit-user-select: none".to_string()}
        unselectable="on"
        onclick={on_click}
        aria-hidden={"true"}
      >
        {content}
      </span>
    }
}
