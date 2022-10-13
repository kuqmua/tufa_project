use yew::Html;
use yew::Properties;
// use crate::components::rc::rc_select::base_select::RawValueType;

// import type * as React from 'react';
// import type { RawValueType } from './BaseSelect';

// export interface FlattenOptionData<OptionType> {
//   label?: React.ReactNode;
//   data: OptionType;
//   key: React.Key;
//   value?: RawValueType;
//   groupOption?: boolean;
//   group?: boolean;
// }

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct FlattenOptionData {
    pub label: Option<Html>,
    // pub data: OptionType;
    pub key: String,
    // pub value: Option<RawValueType>,
    pub group_option: Option<()>,
    pub group: Option<()>,
}
