// import * as React from 'react';
// import type { RadioGroupContextProps, RadioOptionTypeContextProps } from './interface';

// const RadioGroupContext = React.createContext<RadioGroupContextProps | null>(null);

// export const RadioGroupContextProvider = RadioGroupContext.Provider;

// export default RadioGroupContext;

// export const RadioOptionTypeContext = React.createContext<RadioOptionTypeContextProps | null>(null);
// export const RadioOptionTypeContextProvider = RadioOptionTypeContext.Provider;

// import classNames from 'classnames';
// import useMergedState from 'rc-util/lib/hooks/useMergedState';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import SizeContext from '../config-provider/SizeContext';
// import getDataOrAriaProps from '../_util/getDataOrAriaProps';
// import { RadioGroupContextProvider } from './context';
// import type { RadioChangeEvent, RadioGroupButtonStyle, RadioGroupProps } from './interface';
// import Radio from './radio';

// const RadioGroup = React.forwardRef<HTMLDivElement, RadioGroupProps>((props, ref) => {
//   const { getPrefixCls, direction } = React.useContext(ConfigContext);
//   const size = React.useContext(SizeContext);

//   const [value, setValue] = useMergedState(props.defaultValue, {
//     value: props.value,
//   });

//   const onRadioChange = (ev: RadioChangeEvent) => {
//     const lastValue = value;
//     const val = ev.target.value;
//     if (!('value' in props)) {
//       setValue(val);
//     }
//     const { onChange } = props;
//     if (onChange && val !== lastValue) {
//       onChange(ev);
//     }
//   };

//   const renderGroup = () => {
//     const {
//       prefixCls: customizePrefixCls,
//       className = '',
//       options,
//       buttonStyle = 'outline' as RadioGroupButtonStyle,
//       disabled,
//       children,
//       size: customizeSize,
//       style,
//       id,
//       onMouseEnter,
//       onMouseLeave,
//     } = props;
//     const prefixCls = getPrefixCls('radio', customizePrefixCls);
//     const groupPrefixCls = `${prefixCls}-group`;
//     let childrenToRender = children;
//     // 如果存在 options, 优先使用
//     if (options && options.length > 0) {
//       childrenToRender = options.map(option => {
//         if (typeof option === 'string' || typeof option === 'number') {
//           // 此处类型自动推导为 string
//           return (
//             <Radio
//               key={option.toString()}
//               prefixCls={prefixCls}
//               disabled={disabled}
//               value={option}
//               checked={value === option}
//             >
//               {option}
//             </Radio>
//           );
//         }
//         // 此处类型自动推导为 { label: string value: string }
//         return (
//           <Radio
//             key={`radio-group-value-options-${option.value}`}
//             prefixCls={prefixCls}
//             disabled={option.disabled || disabled}
//             value={option.value}
//             checked={value === option.value}
//             style={option.style}
//           >
//             {option.label}
//           </Radio>
//         );
//       });
//     }

//     const mergedSize = customizeSize || size;
//     const classString = classNames(
//       groupPrefixCls,
//       `${groupPrefixCls}-${buttonStyle}`,
//       {
//         [`${groupPrefixCls}-${mergedSize}`]: mergedSize,
//         [`${groupPrefixCls}-rtl`]: direction === 'rtl',
//       },
//       className,
//     );
//     return (
//       <div
//         {...getDataOrAriaProps(props)}
//         className={classString}
//         style={style}
//         onMouseEnter={onMouseEnter}
//         onMouseLeave={onMouseLeave}
//         id={id}
//         ref={ref}
//       >
//         {childrenToRender}
//       </div>
//     );
//   };

//   return (
//     <RadioGroupContextProvider
//       value={{
//         onChange: onRadioChange,
//         value,
//         disabled: props.disabled,
//         name: props.name,
//         optionType: props.optionType,
//       }}
//     >
//       {renderGroup()}
//     </RadioGroupContextProvider>
//   );
// });

// export default React.memo(RadioGroup);

// import type * as React from 'react';
// import Group from './group';
// import type { RadioProps } from './interface';
// import InternalRadio from './radio';
// import Button from './radioButton';

// export {
//   RadioChangeEvent,
//   RadioChangeEventTarget,
//   RadioGroupButtonStyle,
//   RadioGroupContextProps,
//   RadioGroupOptionType,
//   RadioGroupProps,
//   RadioProps,
// } from './interface';
// export { Button, Group };
// interface CompoundedComponent
//   extends React.ForwardRefExoticComponent<RadioProps & React.RefAttributes<HTMLElement>> {
//   Group: typeof Group;
//   Button: typeof Button;
// }

// const Radio = InternalRadio as CompoundedComponent;
// Radio.Button = Button;
// Radio.Group = Group;
// export default Radio;

// import type * as React from 'react';
// import type { AbstractCheckboxProps } from '../checkbox/Checkbox';
// import type { AbstractCheckboxGroupProps } from '../checkbox/Group';
// import type { DisabledType } from '../config-provider/DisabledContext';
// import type { SizeType } from '../config-provider/SizeContext';

// export type RadioGroupButtonStyle = 'outline' | 'solid';
// export type RadioGroupOptionType = 'default' | 'button';

// export interface RadioGroupProps extends AbstractCheckboxGroupProps {
//   defaultValue?: any;
//   value?: any;
//   onChange?: (e: RadioChangeEvent) => void;
//   size?: SizeType;
//   disabled?: DisabledType;
//   onMouseEnter?: React.MouseEventHandler<HTMLDivElement>;
//   onMouseLeave?: React.MouseEventHandler<HTMLDivElement>;
//   name?: string;
//   children?: React.ReactNode;
//   id?: string;
//   optionType?: RadioGroupOptionType;
//   buttonStyle?: RadioGroupButtonStyle;
// }

// export interface RadioGroupContextProps {
//   onChange: (e: RadioChangeEvent) => void;
//   value: any;
//   disabled?: boolean;
//   name?: string;
//   /**
//    * Control the appearance for Radio to display as button or not
//    *
//    * @default 'default'
//    * @internal
//    */
//   optionType?: RadioGroupOptionType;
// }

// export type RadioProps = AbstractCheckboxProps<RadioChangeEvent>;

// export interface RadioChangeEventTarget extends RadioProps {
//   checked: boolean;
// }

// export interface RadioChangeEvent {
//   target: RadioChangeEventTarget;
//   stopPropagation: () => void;
//   preventDefault: () => void;
//   nativeEvent: MouseEvent;
// }

// export type RadioOptionTypeContextProps = RadioGroupOptionType;

// import classNames from 'classnames';
// import RcCheckbox from 'rc-checkbox';
// import { composeRef } from 'rc-util/lib/ref';
// import * as React from 'react';
// import { useContext } from 'react';
// import { ConfigContext } from '../config-provider';
// import DisabledContext from '../config-provider/DisabledContext';
// import { FormItemInputContext } from '../form/context';
// import warning from '../_util/warning';
// import RadioGroupContext, { RadioOptionTypeContext } from './context';
// import type { RadioChangeEvent, RadioProps } from './interface';

// const InternalRadio: React.ForwardRefRenderFunction<HTMLElement, RadioProps> = (props, ref) => {
//   const groupContext = React.useContext(RadioGroupContext);
//   const radioOptionTypeContext = React.useContext(RadioOptionTypeContext);

//   const { getPrefixCls, direction } = React.useContext(ConfigContext);
//   const innerRef = React.useRef<HTMLElement>();
//   const mergedRef = composeRef(ref, innerRef);
//   const { isFormItemInput } = useContext(FormItemInputContext);

//   warning(!('optionType' in props), 'Radio', '`optionType` is only support in Radio.Group.');

//   const onChange = (e: RadioChangeEvent) => {
//     props.onChange?.(e);
//     groupContext?.onChange?.(e);
//   };

//   const {
//     prefixCls: customizePrefixCls,
//     className,
//     children,
//     style,
//     disabled: customDisabled,
//     ...restProps
//   } = props;
//   const radioPrefixCls = getPrefixCls('radio', customizePrefixCls);
//   const prefixCls =
//     (groupContext?.optionType || radioOptionTypeContext) === 'button'
//       ? `${radioPrefixCls}-button`
//       : radioPrefixCls;

//   const radioProps: RadioProps = { ...restProps };

//   // ===================== Disabled =====================
//   const disabled = React.useContext(DisabledContext);
//   radioProps.disabled = customDisabled || disabled;

//   if (groupContext) {
//     radioProps.name = groupContext.name;
//     radioProps.onChange = onChange;
//     radioProps.checked = props.value === groupContext.value;
//     radioProps.disabled = radioProps.disabled || groupContext.disabled;
//   }
//   const wrapperClassString = classNames(
//     `${prefixCls}-wrapper`,
//     {
//       [`${prefixCls}-wrapper-checked`]: radioProps.checked,
//       [`${prefixCls}-wrapper-disabled`]: radioProps.disabled,
//       [`${prefixCls}-wrapper-rtl`]: direction === 'rtl',
//       [`${prefixCls}-wrapper-in-form-item`]: isFormItemInput,
//     },
//     className,
//   );

//   return (
//     // eslint-disable-next-line jsx-a11y/label-has-associated-control
//     <label
//       className={wrapperClassString}
//       style={style}
//       onMouseEnter={props.onMouseEnter}
//       onMouseLeave={props.onMouseLeave}
//     >
//       <RcCheckbox {...radioProps} type="radio" prefixCls={prefixCls} ref={mergedRef} />
//       {children !== undefined ? <span>{children}</span> : null}
//     </label>
//   );
// };

// const Radio = React.forwardRef<unknown, RadioProps>(InternalRadio);

// if (process.env.NODE_ENV !== 'production') {
//   Radio.displayName = 'Radio';
// }

// export default Radio;

// import * as React from 'react';
// import type { AbstractCheckboxProps } from '../checkbox/Checkbox';
// import { ConfigContext } from '../config-provider';
// import { RadioOptionTypeContextProvider } from './context';
// import type { RadioChangeEvent } from './interface';
// import Radio from './radio';

// export type RadioButtonProps = AbstractCheckboxProps<RadioChangeEvent>;

// const RadioButton = (props: RadioButtonProps, ref: React.Ref<any>) => {
//   const { getPrefixCls } = React.useContext(ConfigContext);

//   const { prefixCls: customizePrefixCls, ...radioProps } = props;
//   const prefixCls = getPrefixCls('radio', customizePrefixCls);

//   return (
//     <RadioOptionTypeContextProvider value="button">
//       <Radio prefixCls={prefixCls} {...radioProps} type="radio" ref={ref} />
//     </RadioOptionTypeContextProvider>
//   );
// };

// export default React.forwardRef(RadioButton);
