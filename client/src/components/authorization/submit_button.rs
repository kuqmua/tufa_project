use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubmitButtonProps {
    pub placeholder: Option<String>,
    pub action: Option<Callback<FocusEvent>>,
}
pub struct SubmitButton {}

impl Component for SubmitButton {
    type Message = ();
    type Properties = SubmitButtonProps;
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
        html! {
          <button
            // onclick={ctx.link.callback(|_| log!("submit button click"))}
            // onclick={ctx.props().action}
            tabindex="0"
            type="submit"
            style="
              -webkit-font-smoothing: antialiased;
              border: 0;
              cursor: pointer;
              display: inline-flex;
              outline: 0;
              position: relative;
              align-items: center;
              user-select: none;
              vertical-align: middle;
              justify-content: center;
              text-decoration: none;
              -webkit-appearance: none;
              -webkit-tap-highlight-color: transparent;
              padding: 6px 16px;
              font-size: 0.875rem;
              min-width: 64px;
              box-sizing: border-box;
              transition: background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,border 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
              font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
              font-weight: 500;
              line-height: 1.75;
              border-radius: 4px;
              letter-spacing: 0.02857em;
              text-transform: uppercase;
              box-shadow: 0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12);
              color: #fff;
              background-color: #556cd6;
              width: 100%;
              margin: 24px 0px 16px;
            "
          >
            <span
              style="
                -webkit-font-smoothing: antialiased;
                cursor: pointer;
                user-select: none;
                -webkit-tap-highlight-color: transparent;
                font-size: 0.875rem;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 500;
                line-height: 1.75;
                letter-spacing: 0.02857em;
                text-transform: uppercase;
                color: #fff;
                box-sizing: inherit;
                width: 100%;
                display: inherit;
                align-items: inherit;
                justify-content: inherit;
              "
            >
              {ctx.props().placeholder.clone().unwrap_or("submit button".to_string())}
            </span>
          </button>
        }
    }
}
