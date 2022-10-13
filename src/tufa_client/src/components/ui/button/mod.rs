use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub placeholder: Option<String>,

    pub action: Option<Callback<FocusEvent>>,
}
pub struct Button {}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // the value has changed so we need to
        // re-render for it to appear on the page
        true
    }
    // false
    //https://codepen.io/shawnc8160/pen/xxRYOWg
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        //for some reason page re renders if it would be button

        let text = match ctx.props().placeholder.clone() {
            Some(text) => text,
            None => String::from("Button"),
        };

        html! {
          <button
            tabindex="0"
            type="submit"
            id="btn"
            style={"background:#FF0000"}
          >
              {text}
          </button>
        }
    }
}
