use crate::components::header::buttons::menu_button::MenuButton;
use crate::components::header::buttons::person_outline_button::PersonOutlineButton;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::constants::HEADER_HEIGHT_PX;
use crate::constants::INTERFACE_LINES_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub left_drawer_callback: Callback<MouseEvent>,
    pub right_drawer_callback: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    // let profile_actions_panel_opened = use_state(|| false);
    // let profile_actions_panel_opened_cloned = profile_actions_panel_opened.clone();
    // let change_profile_actions_panel_opened: Callback<MouseEvent> = Callback::from(move |_| {
    //     profile_actions_panel_opened_cloned.set(!*profile_actions_panel_opened_cloned);
    // });
    let header_div_style_handle = format!(
        "
          height: {}px; 
          border-bottom: {}px solid {};
          display: flex;
          flex-direction: column;
        ",
        HEADER_HEIGHT_PX, HEADER_BORDER_BOTTOM_PX, INTERFACE_LINES_COLOR
    );
    html! {
      <header
        style="
          width: 100%;
          min-width: 250px;
          background-color: #16202A;
          position: fixed;
          display: flex;
          flex-direction: column;
        ">
          <div
            style={header_div_style_handle}
          >
            <div
              style="
                display: flex;
                height: 100%; 
                justify-content: space-between; 
                align-items: center;
                padding-right: 20px;
                padding-left: 20px;
              "
            >
              <MenuButton callback={&props.left_drawer_callback}/>
              <div
                style="
                  font-size: 30px;
                  font-family: 'Koulen', cursive;
                  color: white;
                "
              >
                {"Tufa Client"}
              </div>
          // //   <Link<Routes> to={Routes::SignUp}>{ "sign up" }</Link<Routes>>
          // //     {"----------"}
          // //   <Link<Routes> to={Routes::SignIn}>{ "sign ip" }</Link<Routes>>
              <PersonOutlineButton callback={&props.right_drawer_callback}/>
            </div>
          </div>
          // if *profile_actions_panel_opened {
          //   <ProfileActions/>
          // }
      </header>
    }
}
