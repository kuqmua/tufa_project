use crate::components::header::profile_actions::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::logout::Logout;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct LogoutButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(LogoutButton)]
pub fn logout_button(props: &LogoutButtonProps) -> Html {
    let icon_size = "22px".to_owned();
    let html_handle =
        html! {<Logout height={icon_size.clone()} width={icon_size} fill={"#ffffff".to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} placeholder={"Log Out".to_owned()}/>
    }
}
