use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        height={props.height.clone()}
        width={props.width.clone()}
        fill={props.fill.clone()}
      >
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
      </svg>
    }
}
