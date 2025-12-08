use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct WarningProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}

#[function_component(Warning)]
pub fn warning(props: &WarningProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" height={props.height.clone()}  width={props.width.clone()} fill={props.fill.clone()}>
          <path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/>
        </svg>
    }
}
