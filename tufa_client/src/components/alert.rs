#[yew::function_component(Alert)]
pub fn alert() -> yew::html::Html {
    yew::html! {
      <div
        style="
          width: 100%;
          height: 100%;
          color: white;
          display: flex;
          justify-content: center;
          align-items: center;
        "
      >
        {"user must not see this interface. if he see it then there is bug in the code"}
      </div>
    }
}
