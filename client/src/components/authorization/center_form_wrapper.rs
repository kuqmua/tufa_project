use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct CenterFormWrapperProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CenterFormWrapper)]
pub fn center_form_wrapper(props: &CenterFormWrapperProps) -> Html {
    //for some reason height: 100%; is not working
    html! {
      <div
        style="
          width: 100%;
          height: 100vh;
          display: flex;
          justify-content: center;
          align-items: center;
        "
      >
        { for props.children.iter() }
      </div>
    }
}
