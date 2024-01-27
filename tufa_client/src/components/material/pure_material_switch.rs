use yew::{function_component, html};

#[function_component(PureMaterialSwitch)]
pub fn pure_material_switch() -> Html {
    html! {
      <label class="pure-material-switch">
        <input type="checkbox"/>
        <span>{"Switch"}</span>
      </label>
    }
}
