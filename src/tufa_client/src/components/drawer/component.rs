use crate::components::drawer::buttons::close_button::CloseButton;
use crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState;
use crate::components::drawer::position::DrawerPosition;
use crate::constants::BACKGROUND_COLOR;
use crate::constants::FEED_WIDTH_PX;
use web_sys::MouseEvent;
use yew::Html;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub callback: Callback<MouseEvent>,
    pub style_state: DrawerChangingStyleState,
    pub drawer_position: DrawerPosition,
    pub inner_html: Html,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    //todo: add esc keydown handling support(from working drawer.html)
    let drawer_position_style = props.drawer_position.get_style();
    let changing_style = props
        .style_state
        .get_value(drawer_position_style.translate_sign);
    let section_style = format!(
        "
        display: {};
      ",
        changing_style.display
    );
    let drawer_overlay_style = format!(
        "
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        width: 100%;
        z-index: 200;
        opacity: 0;
        transition: opacity 0.3s;
        will-change: opacity;
        background-color: #000;
        -webkit-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        user-select: none;
        opacity: {};
      ",
        changing_style.opacity
    );
    let drawer_wrapper_style = format!(
        "
        position: fixed;
        top: 0;
        left: {};
        right: {};
        bottom: 0;
        height: 100%;
        width: 100%;
        width: {}px;
        z-index: 9999;
        overflow: auto;
        transition: transform 0.3s;
        will-change: transform;
        background-color: {};
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        -webkit-overflow-scrolling: touch; /* enables momentum scrolling in iOS overflow elements */
        -webkit-transform: {};
        transform: {};
      ",
        drawer_position_style.left_value,
        drawer_position_style.right_value,
        FEED_WIDTH_PX,
        BACKGROUND_COLOR,
        changing_style.webkit_transform,
        changing_style.transform
    );
    let justify_content_value = match props.drawer_position {
        DrawerPosition::Left => String::from("flex-end"),
        DrawerPosition::Right => String::from("flex-start"),
    };
    let close_button_wrapper_style = format!(
        "
        display: flex;
        justify-content: {};
        flex-direction: row;
        width: 100%;
      ",
        justify_content_value
    );
    html! {
      <>
        <section
          style={section_style}
        >
          <div
            style={drawer_overlay_style}
            onclick={&props.callback.clone()}
          >
          </div>
          <div
            style={drawer_wrapper_style}
          >
            <div
              style={close_button_wrapper_style}
            >
              <CloseButton callback={&props.callback.clone()}/>
            </div>
            {props.inner_html.clone()}
          </div>
        </section>
      </>
    }
}
