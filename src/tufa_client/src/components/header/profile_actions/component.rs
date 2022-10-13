use crate::components::header::profile_actions::buttons::logout_button::LogoutButton;
use crate::components::header::profile_actions::buttons::settings_button::SettingsButton;
use crate::constants::BACKGROUND_COLOR;
use crate::constants::DEFAULT_PADDING_PX;
use crate::constants::INTERFACE_LINES_COLOR;
use crate::routes::routes::Routes;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};
use yew_router::history::History;
use yew_router::hooks::use_history;

#[function_component(ProfileActions)]
pub fn profile_actions() -> Html {
    let padding: u32 = 13;
    //todo add assertion here;
    let padding_bottom = padding - DEFAULT_PADDING_PX;
    let style_handle = format!(
        "
        position: absolute;
        top: 43px;
        right: 0px;
        height: fit-content;
        width: 190px;
        border-radius: 0px 0px 0px 20px;
        border-left: 1px solid {};
        border-bottom: 1px solid {};
        background-color: {};
        display: flex;
        justify-content: space-evenly;
        flex-direction: column;
        padding: 13px;
        padding-top: 13px;
        padding-left: 13px;
        padding-right: 13px;
        padding-bottom: {}px;
      ",
        INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR, BACKGROUND_COLOR, padding_bottom
    );
    let history = use_history().unwrap();
    let go_to_sign_in: Callback<MouseEvent> = Callback::once(move |_| {
        history.push(Routes::SignIn);
    });
    let go_to_settings: Callback<MouseEvent> = Callback::once(move |_| {
        // history.push(Routes::Settings);
    });
    html! {
      <div
        style={style_handle}
      >
        <SettingsButton callback={go_to_settings}/>
        <LogoutButton callback={go_to_sign_in}/>
      </div>
    }
}
