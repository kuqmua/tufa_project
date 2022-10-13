use crate::components::drawer::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::close_black::CloseBlack;
use crate::constants::HEADER_ICONS_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct CloseButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(CloseButton)]
pub fn close_button(props: &CloseButtonProps) -> Html {
    let icon_size = "35px".to_owned();
    let html_handle = html! {<CloseBlack height={icon_size.clone()} width={icon_size} fill={HEADER_ICONS_COLOR.to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} id={"menu_button".to_owned()}/>
    }
}
