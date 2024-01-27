use yew::prelude::*;

#[function_component(SetTimeoutExample)]
pub fn set_timeout_example() -> Html {
    let should_say_hi = use_state(|| true);
    let should_say_hi_cloned = should_say_hi.clone();
    gloo::timers::callback::Timeout::new(5000, move || {
        should_say_hi_cloned.set(false);
    })
    .forget();
    html! {
      <div>
        <h1>{"Set Timeout example"}</h1>
        if *should_say_hi {
            <div>{"i say hi"}</div>
        }
      </div>
    }
}
