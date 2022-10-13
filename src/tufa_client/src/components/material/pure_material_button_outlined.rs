use yew::{function_component, html};

#[function_component(PureMaterialButtonOutlined)]
pub fn pure_material_button_outlined() -> Html {
    html! {
      <button class="pure-material-button-outlined">{"Button"}</button>
    }
}
