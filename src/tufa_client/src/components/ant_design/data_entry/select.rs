use gloo::console::log;
use web_sys::window;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::html::onchange::Event;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::Properties;
use yew::UseStateHandle;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct SelectProps {
    pub values: Vec<String>,
    pub default_value: String,
    pub style: Option<String>,
    pub additional_classes: Option<String>,
    pub set_choosen_value: Callback<(MouseEvent, Option<String>)>,
    pub id: String,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let is_open = use_state(|| false);
    let choosen_value: UseStateHandle<String> = use_state(|| props.default_value.clone());
    let choosen_value_cloned = choosen_value.clone();
    let choosen_value_second_cloned = choosen_value.clone();
    let set_choosen_value_cloned = props.set_choosen_value.clone();
    let genereted_id = format!("select-{}", props.id.clone());
    let genereted_id_cloned = genereted_id.clone();
    let on_open = Callback::<MouseEvent>::from(move |e: MouseEvent| {
        // e.prevent_default();
        log!("wtf");
        is_open.set(!*is_open.clone());
        //         let value = &*choosen_value_second_cloned.clone();
        //
        //or maybe https://docs.rs/yew/0.9.1/yew/components/select/index.html
        //HtmlSelectElement
        //selected_index
        //https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlSelectElement.html#method.selected_index
        // let value: Option<String> = match window() {
        //     None => None,
        //     Some(window) => match window.document() {
        //         None => None,
        //         Some(document) => {
        //             match document.query_selector(&format!("#{}", genereted_id_cloned)) {
        //                 //something to test creation dom method, no actual need in created element
        //                 Err(e) => {
        //                     log!("eeerror ", e);
        //                     None
        //                 }
        //                 Ok(option_element) => match option_element {
        //                     None => {
        //                         log!("none");
        //                         None
        //                     }
        //                     Some(element) => {
        //                         log!("element");
        //                         let b = element.selected_index();
        //                         None
        //                     }
        //                 },
        //             }
        //         }
        //     },
        // };
        //
        set_choosen_value_cloned.emit((e, None));
    });
    //     let on_click = Callback::<MouseEvent>::from(move |e: MouseEvent| {
    //         log!("oooo");
    //     });
    let options = props
        .values
        .iter()
        .enumerate()
        .map(|(index, v)| {
            if v.clone() == *choosen_value_cloned.clone() {
                html! {
                    <option id={format!("{}{}", genereted_id.clone(), v.clone())} selected={true} value={index.to_string()}>{v}</option>
                }
            } else {
                html! {
                    <option id={format!("{}{}", genereted_id.clone(), v.clone())} value={index.to_string()}>{v}</option>
                }
            }
        })
        .collect::<Html>();
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let classes = match props.additional_classes.clone() {
        None => String::from("form-select"),
        Some(ac) => format!("form-select {}", ac),
    };
    html! {
          <select
            id={genereted_id.clone()}
            onclick={on_open.clone()}
    //         onchange={on_open.clone()}
            class={classes}
            style={style}
          >
            {options}
          </select>
        }
}

// <form>
//     <select id="select_id">
//         <option value="1">11</option>
//         <option value="2">22</option>
//     </select>
//     <button id="btn">Get the Selected Index</button>
// </form>
// <script>
//     const btn = document.querySelector('#btn');
//     const sb = document.querySelector('#select_id')
//     btn.onclick = (event) => {
//         event.preventDefault();
//         alert(sb.selectedIndex);//работает
//     };
// </script>
// ///////////////////////////////////
// let s: Option<i32> = match document.query_selector("select_id")) {
//     Err(e) => None
//     Ok(option_element) => match option_element {
//         None => None,
//         Some(element) => {//тип Element
//             let b = element.selected_index();//не работает
//             None
//         }
//     },
// }
