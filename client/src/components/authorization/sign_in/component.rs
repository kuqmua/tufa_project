use crate::components::authorization::input_form::InputForm;
use crate::components::authorization::svg_icon_wrapper::SvgIconWrapper;
use crate::helpers::html_input_type::HtmlInputType;
use crate::routes::routes::Routes;
use crate::{components::authorization::submit_button::SubmitButton, store::YewduxStore};
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let username = use_state(|| String::from(""));
    let password = use_state(|| String::from(""));
    let username_cloned = username;
    let password_cloned = password;
    let header_name = "Sign in";
    let history = use_history().unwrap();
    let handle_form_submit = store.dispatch().reduce_callback(move |state| {
        state.username = username_cloned.to_string();
        state.password = password_cloned.to_string();
        history.push(Routes::Home);
    });
    let handle_username_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;
            log!("username", state.username.clone());
        });
    let handle_password_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.password = password;
            log!("password", state.password.clone());
        });
    html! {
      <div
        style="
          display: block;
          color: rgba(0, 53, 0, 0.87);
          margin: 0;
          font-size: 0.875rem;
          font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
          font-weight: 400;
          line-height: 1.43;
          letter-spacing: 0.01071em;
          background-color: #fff;
          -webkit-font-smoothing: antialiased;
          box-sizing: inherit;
          border-radius: 10px;
        "
      >
        <div
          style="
            max-width: 444px;
            width: 100%;
            display: block;
            box-sizing: border-box;
            margin-left: auto;
            margin-right: auto;
            padding-left: 16px;
            padding-right: 16px;
          "
        >
          <div
            style="
              display: flex;
              margin-top: 64px;
              align-items: center;
              flex-direction: column;
            "
          >
            <SvgIconWrapper/>
            <h1
              style="
                font-size: 1.5rem;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 400;
                line-height: 1.334;
                letter-spacing: 0em;
                margin: 0;
                box-sizing: inherit;
                display: block;
                font-size: 2em;
                margin-block-start: 0.67em;
                margin-block-end: 0.67em;
                margin-inline-start: 0px;
                margin-inline-end: 0px;
                font-weight: bold;
              "
            >
              {header_name}
            </h1>
            <form
              onsubmit={handle_form_submit}
              novalidate=true
              style="
                width: 100%;
                margin-top: 24px;
                box-sizing: inherit;
                display: block;
              "
            >
              <div
                style="
                  width: calc(100% + 16px);
                  margin: -8px;
                  display: flex;
                  flex-wrap: wrap;
                  box-sizing: border-box;
                "
              >
                <InputForm placeholder={"Login".to_owned()} input_type={HtmlInputType::Text} action={handle_username_change} />
                <InputForm placeholder={"Password".to_owned()} input_type={HtmlInputType::Password} action={handle_password_change.clone()} />
              </div>
              <div>
                <SubmitButton placeholder={header_name}/>
              </div>
              <div
                style="
                  justify-content: center;
                  width: 100%;
                  display: flex;
                  flex-wrap: wrap;
                  box-sizing: border-box;
                  margin-bottom: 10px;
                "
              >
                <div
                  style="
                    margin: 0;
                    box-sizing: border-box;
                    display: block;
                  "
                >
                  <a
                    href=""
                    style="
                      text-decoration: none;
                      color: #556cd6;
                      font-size: 0.875rem;
                      font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                      font-weight: 400;
                      line-height: 1.43;
                      letter-spacing: 0.01071em;
                      margin: 0;
                      box-sizing: inherit;
                      cursor: pointer;
                    "
                  >
                    {"Don't have an account? "}<Link<Routes> to={Routes::SignUp}>{ "Sign up" }</Link<Routes>>
                  </a>
                </div>
              </div>
            </form>
          </div>
        </div>
      </div>
    }
}
