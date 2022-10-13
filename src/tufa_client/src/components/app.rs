use crate::components::common_style::CommonStyle;
use crate::routes::routes::Routes;
use crate::routes::switch_routes::switch_routes;
use yew::{function_component, html};
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <CommonStyle/>
        <Switch<Routes> render={Switch::render(switch_routes)} />
      </BrowserRouter>
    }
}
