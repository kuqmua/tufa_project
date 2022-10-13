use crate::components::material::svg::person_outline::PersonOutline;
use stylist::style;
use stylist::Style;
use yew::html;
use yew::prelude::*;
use yew::Html;

pub enum Msg {
    AddOne,
    Other,
}
pub struct SvgIconWrapper {
    pub stylesheet: Style,
}

impl SvgIconWrapper {
    fn style() -> Style {
        style!(
            "
              margin: 8px;
              background-color: #19857b;
              color: #fff;
              width: 40px;
              height: 40px;
              display: flex;
              overflow: hidden;
              position: relative;
              align-items: center;
              flex-shrink: 0;
              user-select: none;
              border-radius: 50%;
              justify-content: center;
            "
        )
        .unwrap()
    }
}

impl Component for SvgIconWrapper {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div
            class={self.stylesheet.clone()}
          >
            // <svg
            //   style="
            //     width: 75%;
            //     height: 75%;
            //     fill: currentColor;
            //     width: 1em;
            //     height: 1em;
            //     display: inline-block;
            //     font-size: 1.5rem;
            //     transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
            //     flex-shrink: 0;
            //     user-select: none;
            //   "
            //   focusable="false"
            //   viewBox="0 0 24 24"
            //   aria-hidden="true"
            // >
            //   <path
            //     d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"
            //   >
            //   </path>
            // </svg>
            <PersonOutline height={"75%"} width={"75%"} fill={"#ffffff".to_owned()}/>
          </div>
        }
    }
}
