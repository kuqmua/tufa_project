use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub enum CounterMessage {
    ActionOne,
}
pub struct Counter;

impl Component for Counter {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx
            .props()
            .dispatch()
            .reduce_callback(|state| state.count += 1);
        // .reduce_callback_with(|state, event| state.count += 1);
        html! {
          <div>
            <button onclick={onclick}>{"click me"}</button>
          </div>
        }
    }
}
