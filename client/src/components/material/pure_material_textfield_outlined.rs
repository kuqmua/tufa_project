use yew::{function_component, html};

#[function_component(PureMaterialTextfieldOutlined)]
pub fn pure_material_textfield_outlined() -> Html {
    html! {
      <label class="pure-material-textfield-outlined">
        <input placeholder=" "/>
        <span>{"Textfield"}</span>
      </label>
    }
}
