use yew::{function_component, html};

#[function_component(PureMaterialButtonText)]
pub fn pure_material_button_text() -> Html {
    html! {
      <button class="pure-material-button-text">{"Button"}</button>
    }
}
