use crate::store::YewduxStore;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DisplayCount;

impl Component for DisplayCount {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        html! {
          <div>
            <h1>{"DisplayCount"}</h1>
            <p>{format!("Count: {}", count)}</p>
          </div>
        }
    }
}
