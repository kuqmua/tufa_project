use std::collections::HashMap;
use web_sys::FocusEvent;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::Event;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

// import * as React from 'react';
// import classNames from 'classnames';
// import { composeRef } from 'rc-util/lib/ref';
// import { warning } from 'rc-util/lib/warning';

#[derive(Debug, PartialEq, Clone)]
pub enum InputRef {
    HTMLInputElement(Html),
    HTMLTextAreaElement(Html),
}

// type InputRef = HTMLInputElement | HTMLTextAreaElement;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct InputProps {
    pub prefix_cls: String,
    pub id: String,
    pub input_element: Option<Html>, //not actually an option in react
    pub disabled: Option<()>,
    pub auto_focus: Option<()>,
    pub auto_complete: String,
    pub editable: Option<()>,
    pub active_descendant_id: Option<String>,
    pub value: String,
    pub max_length: Option<i32>,
    pub open: Option<()>,
    pub tab_index: i32,
    /** Pass accessibility props to input */
    pub attrs: HashMap<String, String>, //Record<string, unknown>,

    pub on_key_down: Option<Callback<KeyboardEvent>>, //React.KeyboardEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>,
    pub on_mouse_down: Option<Callback<MouseEvent>>, //React.MouseEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>,
    // pub on_change: Option<Callback<ChangeData>>, //React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>,
    pub on_paste: Option<Callback<Event>>, //React.ClipboardEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>,
                                           // pub on_composition_start: React.CompositionEventHandler<
                                           //    HTMLInputElement | HTMLTextAreaElement | HTMLElement
                                           // >,
                                           // pub on_composition_end: React.CompositionEventHandler<
                                           //    HTMLInputElement | HTMLTextAreaElement | HTMLElement
                                           //  >,
}

// interface InputProps {
//   prefixCls: string;
//   id: string;
//   inputElement: React.ReactElement;
//   disabled: boolean;
//   autoFocus: boolean;
//   autoComplete: string;
//   editable: boolean;
//   activeDescendantId?: string;
//   value: string;
//   maxLength?: number;
//   open: boolean;
//   tabIndex: number;
//   /** Pass accessibility props to input */
//   attrs: Record<string, unknown>;

//   onKeyDown: React.KeyboardEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>;
//   onMouseDown: React.MouseEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>;
//   onChange: React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>;
//   onPaste: React.ClipboardEventHandler<HTMLInputElement | HTMLTextAreaElement | HTMLElement>;
//   onCompositionStart: React.CompositionEventHandler<
//     HTMLInputElement | HTMLTextAreaElement | HTMLElement
//   >;
//   onCompositionEnd: React.CompositionEventHandler<
//     HTMLInputElement | HTMLTextAreaElement | HTMLElement
//   >;
// }

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_node = match props.input_element.clone() {
        None => html! {<input />},
        Some(ie) => ie,
    };
    html! {}
}

// const Input: React.RefForwardingComponent<InputRef, InputProps> = (
//   {
//     prefixCls,
//     id,
//     inputElement,
//     disabled,
//     tabIndex,
//     autoFocus,
//     autoComplete,
//     editable,
//     activeDescendantId,
//     value,
//     maxLength,
//     onKeyDown,
//     onMouseDown,
//     onChange,
//     onPaste,
//     onCompositionStart,
//     onCompositionEnd,
//     open,
//     attrs,
//   },
//   ref,
// ) => {
//   let inputNode: React.ComponentElement<any, any> = inputElement || <input />;

//   const { ref: originRef, props: originProps } = inputNode;

//   const {
//     onKeyDown: onOriginKeyDown,
//     onChange: onOriginChange,
//     onMouseDown: onOriginMouseDown,
//     onCompositionStart: onOriginCompositionStart,
//     onCompositionEnd: onOriginCompositionEnd,
//     style,
//   } = originProps;

//   warning(
//     !('maxLength' in inputNode.props),
//     `Passing 'maxLength' to input element directly may not work because input in BaseSelect is controlled.`,
//   );

//   inputNode = React.cloneElement(inputNode, {
//     type: 'search',
//     ...originProps,

//     // Override over origin props
//     id,
//     ref: composeRef(ref, originRef as any),
//     disabled,
//     tabIndex,
//     autoComplete: autoComplete || 'off',

//     autoFocus,
//     className: classNames(`${prefixCls}-selection-search-input`, inputNode?.props?.className),

//     role: 'combobox',
//     'aria-expanded': open,
//     'aria-haspopup': 'listbox',
//     'aria-owns': `${id}_list`,
//     'aria-autocomplete': 'list',
//     'aria-controls': `${id}_list`,
//     'aria-activedescendant': activeDescendantId,
//     ...attrs,
//     value: editable ? value : '',
//     maxLength,
//     readOnly: !editable,
//     unselectable: !editable ? 'on' : null,

//     style: { ...style, opacity: editable ? null : 0 },

//     onKeyDown: (event: React.KeyboardEvent<HTMLElement>) => {
//       onKeyDown(event);
//       if (onOriginKeyDown) {
//         onOriginKeyDown(event);
//       }
//     },
//     onMouseDown: (event: React.MouseEvent<HTMLElement>) => {
//       onMouseDown(event);
//       if (onOriginMouseDown) {
//         onOriginMouseDown(event);
//       }
//     },
//     onChange: (event: React.ChangeEvent<HTMLElement>) => {
//       onChange(event);
//       if (onOriginChange) {
//         onOriginChange(event);
//       }
//     },
//     onCompositionStart(event: React.CompositionEvent<HTMLElement>) {
//       onCompositionStart(event);
//       if (onOriginCompositionStart) {
//         onOriginCompositionStart(event);
//       }
//     },
//     onCompositionEnd(event: React.CompositionEvent<HTMLElement>) {
//       onCompositionEnd(event);
//       if (onOriginCompositionEnd) {
//         onOriginCompositionEnd(event);
//       }
//     },
//     onPaste,
//   });

//   return inputNode;
// };

// const RefInput = React.forwardRef<InputRef, InputProps>(Input);
// RefInput.displayName = 'Input';

// export default RefInput;
