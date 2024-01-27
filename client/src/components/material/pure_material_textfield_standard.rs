use yew::{function_component, html};

#[function_component(PureMaterialTextfieldStandard)]
pub fn pure_material_textfield_standard() -> Html {
    html! {
      <label class="pure-material-textfield-standard">
        <input placeholder=" "/>
        <span>{"Textfield"}</span>
      </label>
    }
}
