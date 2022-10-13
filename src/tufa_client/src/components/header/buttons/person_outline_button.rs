use crate::components::header::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::person_outline::PersonOutline;
use crate::constants::HEADER_ICONS_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct PersonOutlineButtonProps {
    pub callback: Callback<MouseEvent>,
}

#[function_component(PersonOutlineButton)]
pub fn person_outline_button(props: &PersonOutlineButtonProps) -> Html {
    let icon_size = "26px".to_owned();
    let html_handle = html! {<PersonOutline height={icon_size.clone()} width={icon_size} fill={HEADER_ICONS_COLOR.to_owned()}/>};
    html! {
      <ButtonWrapper inner_html={html_handle} callback={props.callback.clone()} id={"person_outline_button".to_owned()}/>
    }
}
