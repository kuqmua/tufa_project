use yew::{function_component, html};

#[function_component(PureMaterialButtonContained)]
pub fn pure_material_button_contained() -> Html {
    html! {
      <button class="pure-material-button-contained">{"Button"}</button>
    }
}
