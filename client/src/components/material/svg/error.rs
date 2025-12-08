use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" height={props.height.clone()}  width={props.width.clone()} fill={props.fill.clone()}>
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
    }
}
