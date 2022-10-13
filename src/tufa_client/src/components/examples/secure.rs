use crate::components::examples::text_input::TextInput;
use crate::routes::routes::Routes;
use gloo::console::log;
use impl_display::ImplDisplay;
use lazy_static::__Deref;
use stylist::yew::styled_component;
use stylist::{style, Style};
use web_sys::FocusEvent;
use yew::use_effect;
use yew::use_state;
use yew::ContextProvider;
use yew::{html, Callback, Html, Properties};
use yew_router::hooks::use_history;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SecureProps {
    pub first: String,
    pub color: Color,
    pub on_load: Callback<String>,
    pub onsubmit: Callback<SecureProps>,
}

#[derive(Default, Clone, Debug)]
pub struct SecureState {
    username: String,
}

const STYLE_FILE: &str = include_str!("example.css");

#[derive(Debug, PartialEq, ImplDisplay)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ContextProviderStruct {
    pub data: String,
}
#[styled_component(Secure)]
pub fn secure(props: &SecureProps) -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Routes::SignUp));
    let class = "my_title";
    let option: Option<&str> = Some("kekw");
    let example_list = vec!["one", "two", "three"];
    let example_list_html = vec![
        html! { <div>{"one"}</div>},
        html! { <div>{"two"}</div>},
        html! { <div>{"three"}</div>},
    ];
    let example_list_for_ter = vec!["oneiter", "twoiter", "threeiter"];
    let example_list_for_function = vec!["one_function", "two_function", "three_function"];
    let stylesheet = style!(
        r#"
            background-color: blue;
        "#
    )
    .unwrap();
    let file_stylesheet = Style::new(STYLE_FILE).unwrap();
    props.on_load.emit("i loaded".to_string());
    let state = use_state(SecureState::default);
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |new_username: String| {
        // let mut data = cloned_state.deref().clone();
        // data.username = new_username;
        // log!("username changed", data);
        cloned_state.set(SecureState {
            username: new_username,
            ..cloned_state.deref().clone()
        });
    });
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        log!("onsubmit")
    });
    let context = ContextProviderStruct {
        data: String::from("fff"),
    };
    let first_load = use_state(|| true);
    use_effect(move || {
        if *first_load {
            log!("on first render");
            first_load.set(false);
        }
        log!("just render");
        || {}
    });
    html! {
        <div class={file_stylesheet}>
        {"file_stylesheet"}
            <div class={stylesheet}>{"stylesheet"}</div>
            if class == "my_title" {
                <h1 class={class}>{ "my_title" }</h1>
            }
            else {
                <h1 class={class}>{ "not my_title" }</h1>
            }
            if let Some(option) = option {
                <h1 class={class}>{option}</h1>
            }
            else  {
                <h1 class={class}>{"None"}</h1>
            }
            <h1 class={css!("color: red; background-color: green;")}>{"css! macro"}</h1>
            <h1>{&props.first}</h1>
            <h1>{&props.color.to_string()}</h1>
            {example_list}
            {example_list_html}
            {example_list_for_ter.iter().map(|x| html!{<li>{"iter of"}{x}</li>}).collect::<Html>()}
            {list_to_html(example_list_for_function)}
            <p>{"username: "}{&state.username}</p>
            <button {onclick}>{ "Go Home with history" }</button>
            <form onsubmit={onsubmit}>
                <ContextProvider<ContextProviderStruct> context={context}>
                    <TextInput name={"text_input".to_string()} handle_onchange={username_changed}/>
                </ContextProvider<ContextProviderStruct>>
            </form>
            <Link<Routes> to={Routes::NotFound}>{ "click here to go to not found" }</Link<Routes>>
        </div>
    }
}

pub fn list_to_html(vec: Vec<&str>) -> Vec<Html> {
    vec.iter().map(|x| html! {<li>{x}</li>}).collect()
}
