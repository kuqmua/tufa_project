use crate::components::feed::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::share::Share;
use crate::constants::FEED_ICONS_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct ShareButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(ShareButton)]
pub fn share_button(props: &ShareButtonProps) -> Html {
    let icon_size = "24px".to_owned();
    let html_handle = html! {<Share height={icon_size.clone()} width={icon_size} fill={FEED_ICONS_COLOR.to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()}/>
    }
}
