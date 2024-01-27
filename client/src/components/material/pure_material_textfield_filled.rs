use yew::{function_component, html};

#[function_component(PureMaterialTextfieldFilled)]
pub fn pure_material_textfield_filled() -> Html {
    html! {
      <label class="pure-material-textfield-filled">
        <input placeholder=" "/>
        <span>{"Textfield"}</span>
       </label>
    }
}
