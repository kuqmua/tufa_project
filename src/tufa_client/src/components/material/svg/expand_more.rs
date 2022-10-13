use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ExpandMoreProps {
    pub height: String,
    pub width: String,
    pub fill: String,
}

#[function_component(ExpandMore)]
pub fn expand_more(props: &ExpandMoreProps) -> Html {
    html! {
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" height={props.height.clone()}  width={props.width.clone()} fill={props.fill.clone()}>
        <path d="M24 24H0V0h24v24z" fill="none" opacity=".87"/>
        <path d="M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6-1.41-1.41z"/>
      </svg>
    }
}
