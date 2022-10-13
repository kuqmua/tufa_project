use yew::{function_component, html};

#[function_component(Alert)]
pub fn alert() -> Html {
    html! {
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
