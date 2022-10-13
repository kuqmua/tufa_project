use yew::{function_component, html};

#[function_component(ExpandMoreContent)]
pub fn expand_more_content() -> Html {
    html! {
      <div
        style="
          display: flex;
          justify-content: space-evenly;
          align-items: center;
          flex-direction: column;
          height: 100%;
      ">
        <div
          style="
            color: white
        ">
            {"expand_more_one"}
        </div>
        <div
          style="
            color: white
        ">
          {"expand_more_two"}
        </div>
        <div
          style="
            color: white
        ">
          {"expand_more_three"}
        </div>
      </div>
    }
}
