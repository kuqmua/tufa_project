use crate::components::examples::secure::ContextProviderStruct;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            // .value_of()
            .value();
        handle_onchange.emit(value);
    });
    let secure_context = use_context::<ContextProviderStruct>();
    // let history_option = use_history();
    // let onclick: Callback<MouseEvent> = match history_option {
    //     Some(history) => Callback::once(move |_| history.push(Routes::Secure)),
    //     None => Callback::once(move |_| log!("Update1")),
    // };
    html! {
        <div>
            <input type="text" name={props.name.clone()} onchange={onchange}/>
            <p>{"use context: "}{secure_context.unwrap_or_default().data}</p>
        </div>

    }
}
