use yew::{function_component, html};

#[function_component(PureMaterialSlider)]
pub fn pure_material_slider() -> Html {
    html! {
      <label class="pure-material-slider">
        <input type="range" min="0" max="100"/>
        <span>{"Slider"}</span>
      </label>
    }
}
