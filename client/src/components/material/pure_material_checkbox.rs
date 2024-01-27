use yew::{function_component, html};

#[function_component(PureMaterialCheckbox)]
pub fn pure_material_checkbox() -> Html {
    html! {
      <label class="pure-material-checkbox">
        <input type="checkbox"/>
        <span>{"Checkbox"}</span>
      </label>
    }
}
