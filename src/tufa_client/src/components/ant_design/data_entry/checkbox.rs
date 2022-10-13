// import type * as React from 'react';
// import type { CheckboxProps } from './Checkbox';
// import InternalCheckbox from './Checkbox';
// import Group from './Group';

// export { CheckboxChangeEvent, CheckboxProps } from './Checkbox';
// export { CheckboxGroupProps, CheckboxOptionType } from './Group';

// interface CompoundedComponent
//   extends React.ForwardRefExoticComponent<CheckboxProps & React.RefAttributes<HTMLInputElement>> {
//   Group: typeof Group;
//   __ANT_CHECKBOX: boolean;
// }

// const Checkbox = InternalCheckbox as CompoundedComponent;

// Checkbox.Group = Group;
// Checkbox.__ANT_CHECKBOX = true;

// export default Checkbox;

// //////////////////////

// import classNames from 'classnames';
// import RcCheckbox from 'rc-checkbox';
// import * as React from 'react';
// import { useContext } from 'react';
// import { ConfigContext } from '../config-provider';
// import { FormItemInputContext } from '../form/context';
// import warning from '../_util/warning';
// import { GroupContext } from './Group';
// import DisabledContext from '../config-provider/DisabledContext';

// export interface AbstractCheckboxProps<T> {
//   prefixCls?: string;
//   className?: string;
//   defaultChecked?: boolean;
//   checked?: boolean;
//   style?: React.CSSProperties;
//   disabled?: boolean;
//   onChange?: (e: T) => void;
//   onClick?: React.MouseEventHandler<HTMLElement>;
//   onMouseEnter?: React.MouseEventHandler<HTMLElement>;
//   onMouseLeave?: React.MouseEventHandler<HTMLElement>;
//   onKeyPress?: React.KeyboardEventHandler<HTMLElement>;
//   onKeyDown?: React.KeyboardEventHandler<HTMLElement>;
//   value?: any;
//   tabIndex?: number;
//   name?: string;
//   children?: React.ReactNode;
//   id?: string;
//   autoFocus?: boolean;
//   type?: string;
//   skipGroup?: boolean;
// }

// export interface CheckboxChangeEventTarget extends CheckboxProps {
//   checked: boolean;
// }

// export interface CheckboxChangeEvent {
//   target: CheckboxChangeEventTarget;
//   stopPropagation: () => void;
//   preventDefault: () => void;
//   nativeEvent: MouseEvent;
// }

// export interface CheckboxProps extends AbstractCheckboxProps<CheckboxChangeEvent> {
//   indeterminate?: boolean;
// }

// const InternalCheckbox: React.ForwardRefRenderFunction<HTMLInputElement, CheckboxProps> = (
//   {
//     prefixCls: customizePrefixCls,
//     className,
//     children,
//     indeterminate = false,
//     style,
//     onMouseEnter,
//     onMouseLeave,
//     skipGroup = false,
//     disabled,
//     ...restProps
//   },
//   ref,
// ) => {
//   const { getPrefixCls, direction } = React.useContext(ConfigContext);
//   const checkboxGroup = React.useContext(GroupContext);
//   const { isFormItemInput } = useContext(FormItemInputContext);
//   const contextDisabled = useContext(DisabledContext);
//   const mergedDisabled = disabled || checkboxGroup?.disabled || contextDisabled;

//   const prevValue = React.useRef(restProps.value);

//   React.useEffect(() => {
//     checkboxGroup?.registerValue(restProps.value);
//     warning(
//       'checked' in restProps || !!checkboxGroup || !('value' in restProps),
//       'Checkbox',
//       '`value` is not a valid prop, do you mean `checked`?',
//     );
//   }, []);

//   React.useEffect(() => {
//     if (skipGroup) {
//       return;
//     }
//     if (restProps.value !== prevValue.current) {
//       checkboxGroup?.cancelValue(prevValue.current);
//       checkboxGroup?.registerValue(restProps.value);
//       prevValue.current = restProps.value;
//     }
//     return () => checkboxGroup?.cancelValue(restProps.value);
//   }, [restProps.value]);

//   const prefixCls = getPrefixCls('checkbox', customizePrefixCls);
//   const checkboxProps: CheckboxProps = { ...restProps };
//   if (checkboxGroup && !skipGroup) {
//     checkboxProps.onChange = (...args) => {
//       if (restProps.onChange) {
//         restProps.onChange(...args);
//       }
//       if (checkboxGroup.toggleOption) {
//         checkboxGroup.toggleOption({ label: children, value: restProps.value });
//       }
//     };
//     checkboxProps.name = checkboxGroup.name;
//     checkboxProps.checked = checkboxGroup.value.indexOf(restProps.value) !== -1;
//   }
//   const classString = classNames(
//     {
//       [`${prefixCls}-wrapper`]: true,
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//       [`${prefixCls}-wrapper-checked`]: checkboxProps.checked,
//       [`${prefixCls}-wrapper-disabled`]: mergedDisabled,
//       [`${prefixCls}-wrapper-in-form-item`]: isFormItemInput,
//     },
//     className,
//   );
//   const checkboxClass = classNames({
//     [`${prefixCls}-indeterminate`]: indeterminate,
//   });
//   const ariaChecked = indeterminate ? 'mixed' : undefined;
//   return (
//     // eslint-disable-next-line jsx-a11y/label-has-associated-control
//     <label
//       className={classString}
//       style={style}
//       onMouseEnter={onMouseEnter}
//       onMouseLeave={onMouseLeave}
//     >
//       <RcCheckbox
//         aria-checked={ariaChecked}
//         {...checkboxProps}
//         prefixCls={prefixCls}
//         className={checkboxClass}
//         disabled={mergedDisabled}
//         ref={ref}
//       />
//       {children !== undefined && <span>{children}</span>}
//     </label>
//   );
// };

// const Checkbox = React.forwardRef<unknown, CheckboxProps>(InternalCheckbox);
// if (process.env.NODE_ENV !== 'production') {
//   Checkbox.displayName = 'Checkbox';
// }

// export default Checkbox;

// import classNames from 'classnames';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import type { CheckboxChangeEvent } from './Checkbox';
// import Checkbox from './Checkbox';

// export type CheckboxValueType = string | number | boolean;

// export interface CheckboxOptionType {
//   label: React.ReactNode;
//   value: CheckboxValueType;
//   style?: React.CSSProperties;
//   disabled?: boolean;
//   onChange?: (e: CheckboxChangeEvent) => void;
// }

// export interface AbstractCheckboxGroupProps {
//   prefixCls?: string;
//   className?: string;
//   options?: Array<CheckboxOptionType | string | number>;
//   disabled?: boolean;
//   style?: React.CSSProperties;
// }

// export interface CheckboxGroupProps extends AbstractCheckboxGroupProps {
//   name?: string;
//   defaultValue?: Array<CheckboxValueType>;
//   value?: Array<CheckboxValueType>;
//   onChange?: (checkedValue: Array<CheckboxValueType>) => void;
//   children?: React.ReactNode;
// }

// export interface CheckboxGroupContext {
//   name?: string;
//   toggleOption?: (option: CheckboxOptionType) => void;
//   value?: any;
//   disabled?: boolean;
//   registerValue: (val: string) => void;
//   cancelValue: (val: string) => void;
// }

// export const GroupContext = React.createContext<CheckboxGroupContext | null>(null);

// const InternalCheckboxGroup: React.ForwardRefRenderFunction<HTMLDivElement, CheckboxGroupProps> = (
//   {
//     defaultValue,
//     children,
//     options = [],
//     prefixCls: customizePrefixCls,
//     className,
//     style,
//     onChange,
//     ...restProps
//   },
//   ref,
// ) => {
//   const { getPrefixCls, direction } = React.useContext(ConfigContext);

//   const [value, setValue] = React.useState<CheckboxValueType[]>(
//     restProps.value || defaultValue || [],
//   );
//   const [registeredValues, setRegisteredValues] = React.useState<CheckboxValueType[]>([]);

//   React.useEffect(() => {
//     if ('value' in restProps) {
//       setValue(restProps.value || []);
//     }
//   }, [restProps.value]);

//   const getOptions = () =>
//     options.map(option => {
//       if (typeof option === 'string' || typeof option === 'number') {
//         return {
//           label: option,
//           value: option,
//         };
//       }
//       return option;
//     });

//   const cancelValue = (val: string) => {
//     setRegisteredValues(prevValues => prevValues.filter(v => v !== val));
//   };

//   const registerValue = (val: string) => {
//     setRegisteredValues(prevValues => [...prevValues, val]);
//   };

//   const toggleOption = (option: CheckboxOptionType) => {
//     const optionIndex = value.indexOf(option.value);
//     const newValue = [...value];
//     if (optionIndex === -1) {
//       newValue.push(option.value);
//     } else {
//       newValue.splice(optionIndex, 1);
//     }
//     if (!('value' in restProps)) {
//       setValue(newValue);
//     }
//     const opts = getOptions();
//     onChange?.(
//       newValue
//         .filter(val => registeredValues.indexOf(val) !== -1)
//         .sort((a, b) => {
//           const indexA = opts.findIndex(opt => opt.value === a);
//           const indexB = opts.findIndex(opt => opt.value === b);
//           return indexA - indexB;
//         }),
//     );
//   };

//   const prefixCls = getPrefixCls('checkbox', customizePrefixCls);
//   const groupPrefixCls = `${prefixCls}-group`;

//   const domProps = omit(restProps, ['value', 'disabled']);

//   if (options && options.length > 0) {
//     children = getOptions().map(option => (
//       <Checkbox
//         prefixCls={prefixCls}
//         key={option.value.toString()}
//         disabled={'disabled' in option ? option.disabled : restProps.disabled}
//         value={option.value}
//         checked={value.indexOf(option.value) !== -1}
//         onChange={option.onChange}
//         className={`${groupPrefixCls}-item`}
//         style={option.style}
//       >
//         {option.label}
//       </Checkbox>
//     ));
//   }

//   // eslint-disable-next-line react/jsx-no-constructed-context-values
//   const context = {
//     toggleOption,
//     value,
//     disabled: restProps.disabled,
//     name: restProps.name,
//     // https://github.com/ant-design/ant-design/issues/16376
//     registerValue,
//     cancelValue,
//   };
//   const classString = classNames(
//     groupPrefixCls,
//     {
//       [`${groupPrefixCls}-rtl`]: direction === 'rtl',
//     },
//     className,
//   );
//   return (
//     <div className={classString} style={style} {...domProps} ref={ref}>
//       <GroupContext.Provider value={context}>{children}</GroupContext.Provider>
//     </div>
//   );
// };

// const CheckboxGroup = React.forwardRef<HTMLDivElement, CheckboxGroupProps>(InternalCheckboxGroup);

// export default React.memo(CheckboxGroup);
