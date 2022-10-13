use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct LogoutProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}

#[function_component(Logout)]
pub fn logout(props: &LogoutProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" viewBox="0 0 24 24" height={props.height.clone()}  width={props.width.clone()} fill={props.fill.clone()}>
          <g>
            <path d="M0,0h24v24H0V0z" fill="none"/>
          </g>
          <g>
            <path d="M17,8l-1.41,1.41L17.17,11H9v2h8.17l-1.58,1.58L17,16l4-4L17,8z M5,5h7V3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h7v-2H5V5z"/>
          </g>
        </svg>
    }
}
