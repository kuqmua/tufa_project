use crate::components::authorization::center_form_wrapper::CenterFormWrapper;
use crate::components::authorization::sign_up::component::SignUp;
use yew::{function_component, html, ChildrenWithProps, Properties};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SignUpPageProps {
    #[prop_or_default]
    children: ChildrenWithProps<SignUp>,
}

#[function_component(SignUpPage)]
pub fn sign_up_page(props: &SignUpPageProps) -> Html {
    html! {<CenterFormWrapper>{ for props.children.iter() }</CenterFormWrapper>}
}
