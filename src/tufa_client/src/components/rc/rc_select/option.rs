use yew::Children;

// /* istanbul ignore file */
// import type * as React from 'react';
// import type { DefaultOptionType } from './Select';

#[derive(Debug, PartialEq, Clone)]
pub struct OptionProps {
    //todo
    pub children: Children,
}

// export interface OptionProps extends Omit<DefaultOptionType, 'label'> {
//   children: React.ReactNode;

//   /** Save for customize data */
//   [prop: string]: any; // eslint-disable-line @typescript-eslint/no-explicit-any
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OptionFc {
    //todo
    pub is_select_option: bool,
}

// export interface OptionFC extends React.FC<OptionProps> {
//   /** Legacy for check if is a Option Group */
//   isSelectOption: boolean;
// }

// /** This is a placeholder, not real render in dom */
// const Option: OptionFC = () => null;
// Option.isSelectOption = true;

// export default Option;
