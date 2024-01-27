use crate::store::YewduxStore;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DisplayCredentials {
    pub dispatch: DispatchProps<PersistentStore<YewduxStore>>,
}

impl Component for DisplayCredentials {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        Self {
            dispatch: _dispatch,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = ctx.props().state().username.clone();
        let password = ctx.props().state().password.clone();
        html! {
          <div>
            <h1>{"Display username"}</h1>
            <p>{format!("username: {}", username)}</p>
            <h1>{"Display password"}</h1>
            <p>{format!("password: {}", password)}</p>
          </div>
        }
    }
}
