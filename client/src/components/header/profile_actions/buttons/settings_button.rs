use crate::components::header::profile_actions::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::settings_black::SettingsBlack;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct SettingsButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(SettingsButton)]
pub fn settings_button(props: &SettingsButtonProps) -> Html {
    let icon_size = "22px".to_owned();
    let html_handle = html! {<SettingsBlack height={icon_size.clone()} width={icon_size} fill={"#ffffff".to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} placeholder={"Settings".to_owned()}/>
    }
}
