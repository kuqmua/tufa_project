use crate::components::header::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::menu::Menu;
use crate::constants::HEADER_ICONS_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct MenuButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(MenuButton)]
pub fn menu_button(props: &MenuButtonProps) -> Html {
    let icon_size = "26px".to_owned();
    let html_handle = html! {<Menu height={icon_size.clone()} width={icon_size} fill={HEADER_ICONS_COLOR.to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} id={"menu_button".to_owned()}/>
    }
}
