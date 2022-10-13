use crate::constants::FEED_BUTTONS_BACKGROUND_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub inner_html: Html,
    pub callback: Callback<MouseEvent>,
}

#[function_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    let style_handle = format!(
        "
        border-radius: 10px;
        border: 1px solid {};
        width: 35px;
        height: 35px;
        margin-bottom: 8px;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: {};
      ",
        FEED_BUTTONS_BACKGROUND_COLOR, FEED_BUTTONS_BACKGROUND_COLOR
    );
    html! {
      <button
        style={style_handle}
        onclick={&props.callback}
      >
       {props.inner_html.clone()}
      </button>
    }
}
