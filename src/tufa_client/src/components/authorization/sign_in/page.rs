use crate::components::authorization::center_form_wrapper::CenterFormWrapper;
use crate::components::authorization::sign_in::component::SignIn;
use yew::{function_component, html, ChildrenWithProps, Properties};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SignInPageProps {
    #[prop_or_default]
    children: ChildrenWithProps<SignIn>,
}

#[function_component(SignInPage)]
pub fn sign_in_page(props: &SignInPageProps) -> Html {
    html! {<CenterFormWrapper>{ for props.children.iter() }</CenterFormWrapper>}
}
