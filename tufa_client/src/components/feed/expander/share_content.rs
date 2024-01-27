use yew::{function_component, html};

#[function_component(ShareContent)]
pub fn share_content() -> Html {
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
            {"share_one"}
        </div>
        <div
          style="
            color: white
        ">
          {"share_two"}
        </div>
        <div
          style="
            color: white
        ">
          {"share_three"}
        </div>
      </div>
    }
}
