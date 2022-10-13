use yew::{function_component, html};

#[function_component(PureMaterialRadio)]
pub fn pure_material_radio() -> Html {
    html! {
        <>
          <label class="pure-material-radio">
            <input type="radio" name="group"/>
            <span>{"Radio Option 1"}</span>
          </label>
          <br/>
          <br/>
          <label class="pure-material-radio">
            <input type="radio" name="group"/>
            <span>{"Radio Option 2"}</span>
          </label>
        </>
    }
}
