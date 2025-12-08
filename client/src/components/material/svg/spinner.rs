use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}
// was not in material design
//https://codepen.io/mrrocks/pen/ExLovj
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    html! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 66 66"
        height={props.height.clone()}
        width={props.width.clone()}
        fill={props.fill.clone()}
      >
        <circle class="path" fill="none" stroke-width="6" stroke-linecap="round" cx="33" cy="33" r="30">
        </circle>
      </svg>
    }
}
