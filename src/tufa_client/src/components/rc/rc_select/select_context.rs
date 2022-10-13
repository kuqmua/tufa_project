use crate::components::rc::rc_select::interface::FlattenOptionData;
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
// import type { RawValueType, RenderNode } from './BaseSelect';
// import type { FlattenOptionData } from './interface';
// import type { BaseOptionType, FieldNames, OnActiveValue, OnInternalSelect } from './Select';

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SelectContextProps {
    // pub options: BaseOptionType[],
    // pub flatten_options: FlattenOptionData<BaseOptionType>[],
    // pub on_active_value: OnActiveValue,
    pub default_active_first_option: Option<()>,
    // pub on_select: OnInternalSelect,
    pub menu_item_selected_icon: Option<Html>,
    // pub raw_values: Set<RawValueType>,
    // pub field_names?: FieldNames,
    pub virtual_handle: Option<()>,
    pub list_height: Option<i32>,
    pub list_item_height: Option<i32>,
    pub children_as_data: Option<()>,
}

// // Use any here since we do not get the type during compilation
// export interface SelectContextProps {
//   options: BaseOptionType[];
//   flattenOptions: FlattenOptionData<BaseOptionType>[];
//   onActiveValue: OnActiveValue;
//   defaultActiveFirstOption?: boolean;
//   onSelect: OnInternalSelect;
//   menuItemSelectedIcon?: RenderNode;
//   rawValues: Set<RawValueType>;
//   fieldNames?: FieldNames;
//   virtual?: boolean;
//   listHeight?: number;
//   listItemHeight?: number;
//   childrenAsData?: boolean;
// }

// const SelectContext = React.createContext<SelectContextProps>(null);

// export default SelectContext;
