use crate::constants::BACKGROUND_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub inner_html: Html,
    pub callback: Callback<MouseEvent>,
    pub id: String,
}

#[function_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    let size_px: u32 = 26;
    let style_handle = format!(
        "
        width: {}px;
        height: {}px;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: {};
        border: 1px solid {};
        padding: 0px;
      ",
        size_px, size_px, BACKGROUND_COLOR, BACKGROUND_COLOR
    );
    html! {
      <button
        style={style_handle}
        onclick={props.callback.clone()}
        id={props.id.clone()}
      >
       {props.inner_html.clone()}
      </button>
    }
}
