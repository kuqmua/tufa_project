use yew::prelude::*;
use yewdux::prelude::PersistentStore;

use crate::store::YewduxStore;
use yewdux_functional::use_store;

#[function_component(YewduxFunctionalDisplayCredentials)]
pub fn yewdux_functional_display_credentials() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let username = store
        .state()
        .map(|state| state.username.clone())
        .unwrap_or_default();
    let password = store
        .state()
        .map(|state| state.password.clone())
        .unwrap_or_default();
    html! {
      <div>
        <h1>{"Display username"}</h1>
        <p>{format!("username: {}", username)}</p>
        <h1>{"Display password"}</h1>
        <p>{format!("password: {}", password)}</p>
      </div>
    }
}
