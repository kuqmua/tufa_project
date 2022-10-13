use crate::constants::ACTIVE_COLOR;
use yew::{function_component, html};

#[function_component(TouchLine)]
pub fn touch_line() -> Html {
    let style_handle = format!(
        "
          background-color: {};
          height: 5px;
          width: 60px;
          border-radius: 3px;
      ",
        ACTIVE_COLOR,
    );
    html! {
      <div
        style={style_handle}
      />
    }
}
