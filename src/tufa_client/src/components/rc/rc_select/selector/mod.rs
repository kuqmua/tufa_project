pub mod input;
pub mod multiple_selector;
pub mod single_selector;
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

// /**
//  * Cursor rule:
//  * 1. Only `showSearch` enabled
//  * 2. Only `open` is `true`
//  * 3. When typing, set `open` to `true` which hit rule of 2
//  *
//  * Accessibility:
//  * - https://www.w3.org/TR/wai-aria-practices/examples/combobox/aria1.1pattern/listbox-combo.html
//  */
// import * as React from 'react';
// import { useRef } from 'react';
// import KeyCode from 'rc-util/lib/KeyCode';
// import type { ScrollTo } from 'rc-virtual-list/lib/List';
// import MultipleSelector from './MultipleSelector';
// import SingleSelector from './SingleSelector';
// import useLock from '../hooks/useLock';
// import type { CustomTagProps, DisplayValueType, Mode, RenderNode } from '../BaseSelect';
// import { isValidateOpenKey } from '../utils/keyUtil';

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct InnerSelectorProps {
    pub prefix_cls: String,
    pub id: String,
    //   mode: Mode;
    pub input_ref: NodeRef,
    pub placeholder: Option<Html>,
    pub disabled: Option<()>,
    pub auto_focus: Option<()>,
    pub auto_complete: Option<String>,
    //   values: DisplayValueType[];
    pub show_search: Option<()>,
    pub search_value: String,
    pub active_descendant_id: Option<String>,
    pub open: Option<()>,
    pub tab_index: Option<i32>,
    pub max_length: Option<i32>,
    pub on_input_key_down: Callback<KeyboardEvent>, //React.KeyboardEventHandler<HTMLInputElement | HTMLTextAreaElement>;
    pub on_input_mouse_down: Callback<MouseEvent>, //React.MouseEventHandler<HTMLInputElement | HTMLTextAreaElement>;
    pub on_input_change: Callback<Event>, //React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement>;
    pub on_input_paste: Callback<Event>, //React.ClipboardEventHandler<HTMLInputElement | HTMLTextAreaElement>;
    pub on_input_composition_start: Callback<Event>, //React.CompositionEventHandler<HTMLInputElement | HTMLTextAreaElement>;
    pub on_input_composition_end: Callback<Event>, //React.CompositionEventHandler<HTMLInputElement | HTMLTextAreaElement>;
}

// export interface InnerSelectorProps {
//   prefixCls: string;
//   id: string;
//   mode: Mode;

//   inputRef: React.Ref<HTMLInputElement | HTMLTextAreaElement>;
//   placeholder?: React.ReactNode;
//   disabled?: boolean;
//   autoFocus?: boolean;
//   autoComplete?: string;
//   values: DisplayValueType[];
//   showSearch?: boolean;
//   searchValue: string;
//   activeDescendantId?: string;
//   open: boolean;
//   tabIndex?: number;
//   maxLength?: number;

//   onInputKeyDown: React.KeyboardEventHandler<HTMLInputElement | HTMLTextAreaElement>;
//   onInputMouseDown: React.MouseEventHandler<HTMLInputElement | HTMLTextAreaElement>;
//   onInputChange: React.ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement>;
//   onInputPaste: React.ClipboardEventHandler<HTMLInputElement | HTMLTextAreaElement>;
//   onInputCompositionStart: React.CompositionEventHandler<HTMLInputElement | HTMLTextAreaElement>;
//   onInputCompositionEnd: React.CompositionEventHandler<HTMLInputElement | HTMLTextAreaElement>;
// }

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct RefSelectorProps {
    pub focus: Callback<()>,
    pub blur: Callback<()>,
    // pub scroll_to?: ScrollTo;
}

// export interface RefSelectorProps {
//   focus: () => void;
//   blur: () => void;
//   scrollTo?: ScrollTo;
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MaxTagCount {
    Number(i32),
    Responsive,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SelectorProps {
    pub id: String,
    pub prefix_cls: String,
    pub show_search: Option<()>,
    pub open: Option<()>,
    /** Display in the Selector value, it's not same as `value` prop */
    // pub values: DisplayValueType[],
    // pub mode: Mode,
    pub search_value: String,
    pub active_value: String,
    pub input_element: Html,
    pub max_length: Option<i32>,

    pub auto_focus: Option<()>,
    pub active_descendant_id: Option<String>,
    pub tab_index: Option<i32>,
    pub disabled: Option<()>,
    pub placeholder: Option<Html>,
    pub remove_icon: Option<Html>,

    pub max_tag_count: Option<MaxTagCount>,
    pub max_tag_text_length: Option<i32>,
    pub max_tag_placeholder: Option<Html>,
    pub tag_render: Option<Callback<Html>>,

    /** Check if `tokenSeparators` contains `\n` or `\r\n` */
    pub token_with_enter: Option<()>,

    // Motion
    pub choice_transition_name: Option<String>,

    pub on_toggle_open: Option<Callback<()>>,
    /** `onSearch` returns go next step boolean to check if need do toggle open */
    pub on_search: Callback<bool>,
    pub on_search_submit: Option<Callback<()>>,
    pub on_remove: Callback<()>,
    pub on_input_key_down: Option<Callback<KeyboardEvent>>,
    pub dom_ref: NodeRef,
}

// export interface SelectorProps {
//   id: string;
//   prefixCls: string;
//   showSearch?: boolean;
//   open: boolean;
//   /** Display in the Selector value, it's not same as `value` prop */
//   values: DisplayValueType[];
//   mode: Mode;
//   searchValue: string;
//   activeValue: string;
//   inputElement: JSX.Element;
//   maxLength?: number;

//   autoFocus?: boolean;
//   activeDescendantId?: string;
//   tabIndex?: number;
//   disabled?: boolean;
//   placeholder?: React.ReactNode;
//   removeIcon?: RenderNode;

//   // Tags
//   maxTagCount?: number | 'responsive';
//   maxTagTextLength?: number;
//   maxTagPlaceholder?: React.ReactNode | ((omittedValues: DisplayValueType[]) => React.ReactNode);
//   tagRender?: (props: CustomTagProps) => React.ReactElement;

//   /** Check if `tokenSeparators` contains `\n` or `\r\n` */
//   tokenWithEnter?: boolean;

//   // Motion
//   choiceTransitionName?: string;

//   onToggleOpen: (open?: boolean) => void;
//   /** `onSearch` returns go next step boolean to check if need do toggle open */
//   onSearch: (searchText: string, fromTyping: boolean, isCompositing: boolean) => boolean;
//   onSearchSubmit?: (searchText: string) => void;
//   onRemove: (value: DisplayValueType) => void;
//   onInputKeyDown?: React.KeyboardEventHandler<HTMLInputElement | HTMLTextAreaElement>;

//   /**
//    * @private get real dom for trigger align.
//    * This may be removed after React provides replacement of `findDOMNode`
//    */
//   domRef: React.Ref<HTMLDivElement>;
// }

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SelectorPropsWithRefSelectorProps {
    pub id: String,
    pub prefix_cls: String,
    pub show_search: Option<()>,
    pub open: Option<()>,
    /** Display in the Selector value, it's not same as `value` prop */
    // pub values: DisplayValueType[],
    // pub mode: Mode,
    pub search_value: String,
    pub active_value: String,
    pub input_element: Html,
    pub max_length: Option<i32>,

    pub auto_focus: Option<()>,
    pub active_descendant_id: Option<String>,
    pub tab_index: Option<i32>,
    pub disabled: Option<()>,
    pub placeholder: Option<Html>,
    pub remove_icon: Option<Html>,

    pub max_tag_count: Option<MaxTagCount>,
    pub max_tag_text_length: Option<i32>,
    pub max_tag_placeholder: Option<Html>,
    pub tag_render: Option<Callback<Html>>,

    /** Check if `tokenSeparators` contains `\n` or `\r\n` */
    pub token_with_enter: Option<()>,

    // Motion
    pub choice_transition_name: Option<String>,

    pub on_toggle_open: Option<Callback<()>>,
    /** `onSearch` returns go next step boolean to check if need do toggle open */
    pub on_search: Callback<bool>,
    pub on_search_submit: Option<Callback<()>>,
    pub on_remove: Callback<()>,
    pub on_input_key_down: Option<Callback<KeyboardEvent>>,
    pub dom_ref: NodeRef,

    //
    pub focus: Callback<()>,
    pub blur: Callback<()>,
    // pub scroll_to?: ScrollTo;
}

#[function_component(Selector)]
pub fn selector(props: &SelectorPropsWithRefSelectorProps) -> Html {
    let on_input_key_down = props.on_input_key_down.clone();
    let on_internal_input_key_down = Callback::from(move |event: KeyboardEvent| {
        let key = event.key();
        let up = String::from("UP");
        let down = String::from("DOWN");
        if key == up {
            event.prevent_default();
        }
        if key == down {
            event.prevent_default();
        }
        if on_input_key_down.clone().is_some() {
            event.prevent_default();
        }
        //     if (which === KeyCode.ENTER && mode === 'tags' && !compositionStatusRef.current && !open) {
        //       // When menu isn't open, OptionList won't trigger a value change
        //       // So when enter is pressed, the tag's input value should be emitted here to let selector know
        //       onSearchSubmit?.((event.target as HTMLInputElement).value);
        //     }

        //     if (isValidateOpenKey(which)) {
        //       onToggleOpen(true);
        //     }
        //   };

        //   /**
        //    * We can not use `findDOMNode` sine it will get warning,
        //    * have to use timer to check if is input element.
        //    */
        //   const onInternalInputMouseDown: React.MouseEventHandler<HTMLInputElement> = () => {
        //     setInputMouseDown(true);
        //   };

        //   // When paste come, ignore next onChange
        //   const pastedTextRef = useRef<string>(null);

        //   const triggerOnSearch = (value: string) => {
        //     if (onSearch(value, true, compositionStatusRef.current) !== false) {
        //       onToggleOpen(true);
        //     }
        //   };

        //   const onInputCompositionStart = () => {
        //     compositionStatusRef.current = true;
        //   };

        //   const onInputCompositionEnd: React.CompositionEventHandler<HTMLInputElement> = (e) => {
        //     compositionStatusRef.current = false;

        //     // Trigger search again to support `tokenSeparators` with typewriting
        //     if (mode !== 'combobox') {
        //       triggerOnSearch((e.target as HTMLInputElement).value);
        //     }
        //   };

        //   const onInputChange: React.ChangeEventHandler<HTMLInputElement> = (event) => {
        //     let {
        //       target: { value },
        //     } = event;

        //     // Pasted text should replace back to origin content
        //     if (tokenWithEnter && pastedTextRef.current && /[\r\n]/.test(pastedTextRef.current)) {
        //       // CRLF will be treated as a single space for input element
        //       const replacedText = pastedTextRef.current
        //         .replace(/[\r\n]+$/, '')
        //         .replace(/\r\n/g, ' ')
        //         .replace(/[\r\n]/g, ' ');
        //       value = value.replace(replacedText, pastedTextRef.current);
        //     }

        //     pastedTextRef.current = null;

        //     triggerOnSearch(value);
        //   };

        //   const onInputPaste: React.ClipboardEventHandler = (e) => {
        //     const { clipboardData } = e;
        //     const value = clipboardData.getData('text');

        //     pastedTextRef.current = value;
        //   };

        //   const onClick = ({ target }) => {
        //     if (target !== inputRef.current) {
        //       // Should focus input if click the selector
        //       const isIE = (document.body.style as any).msTouchAction !== undefined;
        //       if (isIE) {
        //         setTimeout(() => {
        //           inputRef.current.focus();
        //         });
        //       } else {
        //         inputRef.current.focus();
        //       }
        //     }
        //   };

        //   const onMouseDown: React.MouseEventHandler<HTMLElement> = (event) => {
        //     const inputMouseDown = getInputMouseDown();
        //     if (event.target !== inputRef.current && !inputMouseDown) {
        //       event.preventDefault();
        //     }

        //     if ((mode !== 'combobox' && (!showSearch || !inputMouseDown)) || !open) {
        //       if (open) {
        //         onSearch('', true, false);
        //       }
        //       onToggleOpen();
        //     }
        //   };

        //   // ================= Inner Selector ==================
        //   const sharedProps = {
        //     inputRef,
        //     onInputKeyDown: onInternalInputKeyDown,
        //     onInputMouseDown: onInternalInputMouseDown,
        //     onInputChange,
        //     onInputPaste,
        //     onInputCompositionStart,
        //     onInputCompositionEnd,
        //   };

        //   const selectNode =
        //     mode === 'multiple' || mode === 'tags' ? (
        //       <MultipleSelector {...props} {...sharedProps} />
        //     ) : (
        //       <SingleSelector {...props} {...sharedProps} />
        //     );

        //   return (
        //     <div
        //       ref={domRef}
        //       className={`${prefixCls}-selector`}
        //       onClick={onClick}
        //       onMouseDown={onMouseDown}
        //     >
        //       {selectNode}
        //     </div>
        //   );
    });
    html! {}
}

// const Selector: React.RefForwardingComponent<RefSelectorProps, SelectorProps> = (props, ref) => {
//   const inputRef = useRef<HTMLInputElement>(null);
//   const compositionStatusRef = useRef<boolean>(false);

//   const {
//     prefixCls,
//     open,
//     mode,
//     showSearch,
//     tokenWithEnter,

//     onSearch,
//     onSearchSubmit,
//     onToggleOpen,
//     onInputKeyDown,

//     domRef,
//   } = props;

//   // ======================= Ref =======================
//   React.useImperativeHandle(ref, () => ({
//     focus: () => {
//       inputRef.current.focus();
//     },
//     blur: () => {
//       inputRef.current.blur();
//     },
//   }));

//   // ====================== Input ======================
//   const [getInputMouseDown, setInputMouseDown] = useLock(0);

//   const onInternalInputKeyDown: React.KeyboardEventHandler<HTMLInputElement> = (event) => {
//     const { which } = event;

//     if (which === KeyCode.UP || which === KeyCode.DOWN) {
//       event.preventDefault();
//     }

//     if (onInputKeyDown) {
//       onInputKeyDown(event);
//     }

//     if (which === KeyCode.ENTER && mode === 'tags' && !compositionStatusRef.current && !open) {
//       // When menu isn't open, OptionList won't trigger a value change
//       // So when enter is pressed, the tag's input value should be emitted here to let selector know
//       onSearchSubmit?.((event.target as HTMLInputElement).value);
//     }

//     if (isValidateOpenKey(which)) {
//       onToggleOpen(true);
//     }
//   };

//   /**
//    * We can not use `findDOMNode` sine it will get warning,
//    * have to use timer to check if is input element.
//    */
//   const onInternalInputMouseDown: React.MouseEventHandler<HTMLInputElement> = () => {
//     setInputMouseDown(true);
//   };

//   // When paste come, ignore next onChange
//   const pastedTextRef = useRef<string>(null);

//   const triggerOnSearch = (value: string) => {
//     if (onSearch(value, true, compositionStatusRef.current) !== false) {
//       onToggleOpen(true);
//     }
//   };

//   const onInputCompositionStart = () => {
//     compositionStatusRef.current = true;
//   };

//   const onInputCompositionEnd: React.CompositionEventHandler<HTMLInputElement> = (e) => {
//     compositionStatusRef.current = false;

//     // Trigger search again to support `tokenSeparators` with typewriting
//     if (mode !== 'combobox') {
//       triggerOnSearch((e.target as HTMLInputElement).value);
//     }
//   };

//   const onInputChange: React.ChangeEventHandler<HTMLInputElement> = (event) => {
//     let {
//       target: { value },
//     } = event;

//     // Pasted text should replace back to origin content
//     if (tokenWithEnter && pastedTextRef.current && /[\r\n]/.test(pastedTextRef.current)) {
//       // CRLF will be treated as a single space for input element
//       const replacedText = pastedTextRef.current
//         .replace(/[\r\n]+$/, '')
//         .replace(/\r\n/g, ' ')
//         .replace(/[\r\n]/g, ' ');
//       value = value.replace(replacedText, pastedTextRef.current);
//     }

//     pastedTextRef.current = null;

//     triggerOnSearch(value);
//   };

//   const onInputPaste: React.ClipboardEventHandler = (e) => {
//     const { clipboardData } = e;
//     const value = clipboardData.getData('text');

//     pastedTextRef.current = value;
//   };

//   const onClick = ({ target }) => {
//     if (target !== inputRef.current) {
//       // Should focus input if click the selector
//       const isIE = (document.body.style as any).msTouchAction !== undefined;
//       if (isIE) {
//         setTimeout(() => {
//           inputRef.current.focus();
//         });
//       } else {
//         inputRef.current.focus();
//       }
//     }
//   };

//   const onMouseDown: React.MouseEventHandler<HTMLElement> = (event) => {
//     const inputMouseDown = getInputMouseDown();
//     if (event.target !== inputRef.current && !inputMouseDown) {
//       event.preventDefault();
//     }

//     if ((mode !== 'combobox' && (!showSearch || !inputMouseDown)) || !open) {
//       if (open) {
//         onSearch('', true, false);
//       }
//       onToggleOpen();
//     }
//   };

//   // ================= Inner Selector ==================
//   const sharedProps = {
//     inputRef,
//     onInputKeyDown: onInternalInputKeyDown,
//     onInputMouseDown: onInternalInputMouseDown,
//     onInputChange,
//     onInputPaste,
//     onInputCompositionStart,
//     onInputCompositionEnd,
//   };

//   const selectNode =
//     mode === 'multiple' || mode === 'tags' ? (
//       <MultipleSelector {...props} {...sharedProps} />
//     ) : (
//       <SingleSelector {...props} {...sharedProps} />
//     );

//   return (
//     <div
//       ref={domRef}
//       className={`${prefixCls}-selector`}
//       onClick={onClick}
//       onMouseDown={onMouseDown}
//     >
//       {selectNode}
//     </div>
//   );
// };

// const ForwardSelector = React.forwardRef<RefSelectorProps, SelectorProps>(Selector);
// ForwardSelector.displayName = 'Selector';

// export default ForwardSelector;
