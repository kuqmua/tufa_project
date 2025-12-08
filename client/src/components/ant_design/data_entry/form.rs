// import classNames from 'classnames';
// import CSSMotion, { CSSMotionList } from 'rc-motion';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import collapseMotion from '../_util/motion';
// import { FormItemPrefixContext } from './context';
// import type { ValidateStatus } from './FormItem';

// const EMPTY_LIST: React.ReactNode[] = [];

// interface ErrorEntity {
//   error: React.ReactNode;
//   errorStatus?: ValidateStatus;
//   key: string;
// }

// function toErrorEntity(
//   error: React.ReactNode,
//   errorStatus: ValidateStatus | undefined,
//   prefix: string,
//   index: number = 0,
// ): ErrorEntity {
//   return {
//     key: typeof error === 'string' ? error : `${prefix}-${index}`,
//     error,
//     errorStatus,
//   };
// }

// export interface ErrorListProps {
//   help?: React.ReactNode;
//   helpStatus?: ValidateStatus;
//   errors?: React.ReactNode[];
//   warnings?: React.ReactNode[];
//   className?: string;
// }

// export default function ErrorList({
//   help,
//   helpStatus,
//   errors = EMPTY_LIST,
//   warnings = EMPTY_LIST,
//   className: rootClassName,
// }: ErrorListProps) {
//   const { prefixCls } = React.useContext(FormItemPrefixContext);
//   const { getPrefixCls } = React.useContext(ConfigContext);

//   const baseClassName = `${prefixCls}-item-explain`;
//   const rootPrefixCls = getPrefixCls();

//   const fullKeyList = React.useMemo(() => {
//     if (help !== undefined && help !== null) {
//       return [toErrorEntity(help, helpStatus, 'help')];
//     }

//     return [
//       ...errors.map((error, index) => toErrorEntity(error, 'error', 'error', index)),
//       ...warnings.map((warning, index) => toErrorEntity(warning, 'warning', 'warning', index)),
//     ];
//   }, [help, helpStatus, errors, warnings]);

//   return (
//     <CSSMotion
//       {...collapseMotion}
//       motionName={`${rootPrefixCls}-show-help`}
//       motionAppear={false}
//       motionEnter={false}
//       visible={!!fullKeyList.length}
//       onLeaveStart={node => {
//         // Force disable css override style in index.less configured
//         node.style.height = 'auto';
//         return { height: node.offsetHeight };
//       }}
//     >
//       {holderProps => {
//         const { className: holderClassName, style: holderStyle } = holderProps;

//         return (
//           <div
//             className={classNames(baseClassName, holderClassName, rootClassName)}
//             style={holderStyle}
//           >
//             <CSSMotionList
//               keys={fullKeyList}
//               {...collapseMotion}
//               motionName={`${rootPrefixCls}-show-help-item`}
//               component={false}
//             >
//               {itemProps => {
//                 const {
//                   key,
//                   error,
//                   errorStatus,
//                   className: itemClassName,
//                   style: itemStyle,
//                 } = itemProps;

//                 return (
//                   <div
//                     key={key}
//                     role="alert"
//                     className={classNames(itemClassName, {
//                       [`${baseClassName}-${errorStatus}`]: errorStatus,
//                     })}
//                     style={itemStyle}
//                   >
//                     {error}
//                   </div>
//                 );
//               }}
//             </CSSMotionList>
//           </div>
//         );
//       }}
//     </CSSMotion>
//   );
// }

// import classNames from 'classnames';
// import FieldForm, { List, useWatch } from 'rc-field-form';
// import type { FormProps as RcFormProps } from 'rc-field-form/lib/Form';
// import type { ValidateErrorEntity } from 'rc-field-form/lib/interface';
// import * as React from 'react';
// import { useMemo } from 'react';
// import type { Options } from 'scroll-into-view-if-needed';
// import { ConfigContext } from '../config-provider';
// import DisabledContext, { DisabledContextProvider } from '../config-provider/DisabledContext';
// import type { SizeType } from '../config-provider/SizeContext';
// import SizeContext, { SizeContextProvider } from '../config-provider/SizeContext';
// import type { ColProps } from '../grid/col';
// import type { FormContextProps } from './context';
// import { FormContext } from './context';
// import useForm, { FormInstance } from './hooks/useForm';
// import type { FormLabelAlign } from './interface';

// export type RequiredMark = boolean | 'optional';
// export type FormLayout = 'horizontal' | 'inline' | 'vertical';

// export interface FormProps<Values = any> extends Omit<RcFormProps<Values>, 'form'> {
//   prefixCls?: string;
//   colon?: boolean;
//   name?: string;
//   layout?: FormLayout;
//   labelAlign?: FormLabelAlign;
//   labelWrap?: boolean;
//   labelCol?: ColProps;
//   wrapperCol?: ColProps;
//   form?: FormInstance<Values>;
//   size?: SizeType;
//   disabled?: boolean;
//   scrollToFirstError?: Options | boolean;
//   requiredMark?: RequiredMark;
//   /** @deprecated Will warning in future branch. Pls use `requiredMark` instead. */
//   hideRequiredMark?: boolean;
// }

// const InternalForm: React.ForwardRefRenderFunction<FormInstance, FormProps> = (props, ref) => {
//   const contextSize = React.useContext(SizeContext);
//   const contextDisabled = React.useContext(DisabledContext);
//   const { getPrefixCls, direction, form: contextForm } = React.useContext(ConfigContext);

//   const {
//     prefixCls: customizePrefixCls,
//     className = '',
//     size = contextSize,
//     disabled = contextDisabled,
//     form,
//     colon,
//     labelAlign,
//     labelWrap,
//     labelCol,
//     wrapperCol,
//     hideRequiredMark,
//     layout = 'horizontal',
//     scrollToFirstError,
//     requiredMark,
//     onFinishFailed,
//     name,
//     ...restFormProps
//   } = props;

//   const mergedRequiredMark = useMemo(() => {
//     if (requiredMark !== undefined) {
//       return requiredMark;
//     }

//     if (contextForm && contextForm.requiredMark !== undefined) {
//       return contextForm.requiredMark;
//     }

//     if (hideRequiredMark) {
//       return false;
//     }

//     return true;
//   }, [hideRequiredMark, requiredMark, contextForm]);

//   const mergedColon = colon ?? contextForm?.colon;

//   const prefixCls = getPrefixCls('form', customizePrefixCls);

//   const formClassName = classNames(
//     prefixCls,
//     {
//       [`${prefixCls}-${layout}`]: true,
//       [`${prefixCls}-hide-required-mark`]: mergedRequiredMark === false,
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//       [`${prefixCls}-${size}`]: size,
//     },
//     className,
//   );

//   const [wrapForm] = useForm(form);
//   const { __INTERNAL__ } = wrapForm;
//   __INTERNAL__.name = name;

//   const formContextValue = useMemo<FormContextProps>(
//     () => ({
//       name,
//       labelAlign,
//       labelCol,
//       labelWrap,
//       wrapperCol,
//       vertical: layout === 'vertical',
//       colon: mergedColon,
//       requiredMark: mergedRequiredMark,
//       itemRef: __INTERNAL__.itemRef,
//       form: wrapForm,
//     }),
//     [name, labelAlign, labelCol, wrapperCol, layout, mergedColon, mergedRequiredMark, wrapForm],
//   );

//   React.useImperativeHandle(ref, () => wrapForm);

//   const onInternalFinishFailed = (errorInfo: ValidateErrorEntity) => {
//     onFinishFailed?.(errorInfo);

//     let defaultScrollToFirstError: Options = { block: 'nearest' };

//     if (scrollToFirstError && errorInfo.errorFields.length) {
//       if (typeof scrollToFirstError === 'object') {
//         defaultScrollToFirstError = scrollToFirstError;
//       }
//       wrapForm.scrollToField(errorInfo.errorFields[0].name, defaultScrollToFirstError);
//     }
//   };

//   return (
//     <DisabledContextProvider disabled={disabled}>
//       <SizeContextProvider size={size}>
//         <FormContext.Provider value={formContextValue}>
//           <FieldForm
//             id={name}
//             {...restFormProps}
//             name={name}
//             onFinishFailed={onInternalFinishFailed}
//             form={wrapForm}
//             className={formClassName}
//           />
//         </FormContext.Provider>
//       </SizeContextProvider>
//     </DisabledContextProvider>
//   );
// };

// const Form = React.forwardRef<FormInstance, FormProps>(InternalForm) as <Values = any>(
//   props: React.PropsWithChildren<FormProps<Values>> & { ref?: React.Ref<FormInstance<Values>> },
// ) => React.ReactElement;

// export { useForm, List, FormInstance, useWatch };

// export default Form;

// import CheckCircleFilled from '@ant-design/icons/CheckCircleFilled';
// import CloseCircleFilled from '@ant-design/icons/CloseCircleFilled';
// import ExclamationCircleFilled from '@ant-design/icons/ExclamationCircleFilled';
// import LoadingOutlined from '@ant-design/icons/LoadingOutlined';
// import classNames from 'classnames';
// import type { FormInstance } from 'rc-field-form';
// import { Field, FieldContext, ListContext } from 'rc-field-form';
// import type { FieldProps } from 'rc-field-form/lib/Field';
// import type { Meta, NamePath } from 'rc-field-form/lib/interface';
// import useState from 'rc-util/lib/hooks/useState';
// import omit from 'rc-util/lib/omit';
// import { supportRef } from 'rc-util/lib/ref';
// import type { ReactNode } from 'react';
// import * as React from 'react';
// import { useContext, useMemo } from 'react';
// import { ConfigContext } from '../config-provider';
// import Row from '../grid/row';
// import { cloneElement, isValidElement } from '../_util/reactNode';
// import { tuple } from '../_util/type';
// import warning from '../_util/warning';
// import type { FormItemStatusContextProps } from './context';
// import { FormContext, FormItemInputContext, NoStyleItemContext } from './context';
// import type { FormItemInputProps } from './FormItemInput';
// import FormItemInput from './FormItemInput';
// import type { FormItemLabelProps, LabelTooltipType } from './FormItemLabel';
// import FormItemLabel from './FormItemLabel';
// import useDebounce from './hooks/useDebounce';
// import useFrameState from './hooks/useFrameState';
// import useItemRef from './hooks/useItemRef';
// import { getFieldId, toArray } from './util';

// const NAME_SPLIT = '__SPLIT__';

// interface FieldError {
//   errors: string[];
//   warnings: string[];
// }

// const ValidateStatuses = tuple('success', 'warning', 'error', 'validating', '');
// export type ValidateStatus = typeof ValidateStatuses[number];

// type RenderChildren<Values = any> = (form: FormInstance<Values>) => React.ReactNode;
// type RcFieldProps<Values = any> = Omit<FieldProps<Values>, 'children'>;
// type ChildrenType<Values = any> = RenderChildren<Values> | React.ReactNode;

// interface MemoInputProps {
//   value: any;
//   update: any;
//   children: React.ReactNode;
// }

// const MemoInput = React.memo(
//   ({ children }: MemoInputProps) => children as JSX.Element,
//   (prev, next) => prev.value === next.value && prev.update === next.update,
// );

// export interface FormItemProps<Values = any>
//   extends FormItemLabelProps,
//     FormItemInputProps,
//     RcFieldProps<Values> {
//   prefixCls?: string;
//   noStyle?: boolean;
//   style?: React.CSSProperties;
//   className?: string;
//   children?: ChildrenType<Values>;
//   id?: string;
//   hasFeedback?: boolean;
//   validateStatus?: ValidateStatus;
//   required?: boolean;
//   hidden?: boolean;
//   initialValue?: any;
//   messageVariables?: Record<string, string>;
//   tooltip?: LabelTooltipType;
//   /** @deprecated No need anymore */
//   fieldKey?: React.Key | React.Key[];
// }

// function hasValidName(name?: NamePath): Boolean {
//   if (name === null) {
//     warning(false, 'Form.Item', '`null` is passed as `name` property');
//   }
//   return !(name === undefined || name === null);
// }

// function genEmptyMeta(): Meta {
//   return {
//     errors: [],
//     warnings: [],
//     touched: false,
//     validating: false,
//     name: [],
//   };
// }

// const iconMap = {
//   success: CheckCircleFilled,
//   warning: ExclamationCircleFilled,
//   error: CloseCircleFilled,
//   validating: LoadingOutlined,
// };

// function FormItem<Values = any>(props: FormItemProps<Values>): React.ReactElement {
//   const {
//     name,
//     noStyle,
//     dependencies,
//     prefixCls: customizePrefixCls,
//     style,
//     className,
//     shouldUpdate,
//     hasFeedback,
//     help,
//     rules,
//     validateStatus,
//     children,
//     required,
//     label,
//     messageVariables,
//     trigger = 'onChange',
//     validateTrigger,
//     hidden,
//     ...restProps
//   } = props;
//   const { getPrefixCls } = useContext(ConfigContext);
//   const { name: formName, requiredMark } = useContext(FormContext);
//   const isRenderProps = typeof children === 'function';
//   const notifyParentMetaChange = useContext(NoStyleItemContext);

//   const { validateTrigger: contextValidateTrigger } = useContext(FieldContext);
//   const mergedValidateTrigger =
//     validateTrigger !== undefined ? validateTrigger : contextValidateTrigger;

//   const hasName = hasValidName(name);

//   const prefixCls = getPrefixCls('form', customizePrefixCls);

//   // ========================= MISC =========================
//   // Get `noStyle` required info
//   const listContext = React.useContext(ListContext);
//   const fieldKeyPathRef = React.useRef<React.Key[]>();

//   // ======================== Errors ========================
//   // >>>>> Collect sub field errors
//   const [subFieldErrors, setSubFieldErrors] = useFrameState<Record<string, FieldError>>({});

//   // >>>>> Current field errors
//   const [meta, setMeta] = useState<Meta>(() => genEmptyMeta());

//   const onMetaChange = (nextMeta: Meta & { destroy?: boolean }) => {
//     // This keyInfo is not correct when field is removed
//     // Since origin keyManager no longer keep the origin key anymore
//     // Which means we need cache origin one and reuse when removed
//     const keyInfo = listContext?.getKey(nextMeta.name);

//     // Destroy will reset all the meta
//     setMeta(nextMeta.destroy ? genEmptyMeta() : nextMeta, true);

//     // Bump to parent since noStyle
//     if (noStyle && notifyParentMetaChange) {
//       let namePath = nextMeta.name;

//       if (!nextMeta.destroy) {
//         if (keyInfo !== undefined) {
//           const [fieldKey, restPath] = keyInfo;
//           namePath = [fieldKey, ...restPath];
//           fieldKeyPathRef.current = namePath;
//         }
//       } else {
//         // Use origin cache data
//         namePath = fieldKeyPathRef.current || namePath;
//       }
//       notifyParentMetaChange(nextMeta, namePath);
//     }
//   };

//   // >>>>> Collect noStyle Field error to the top FormItem
//   const onSubItemMetaChange = (subMeta: Meta & { destroy: boolean }, uniqueKeys: React.Key[]) => {
//     // Only `noStyle` sub item will trigger
//     setSubFieldErrors(prevSubFieldErrors => {
//       const clone = {
//         ...prevSubFieldErrors,
//       };

//       // name: ['user', 1] + key: [4] = ['user', 4]
//       const mergedNamePath = [...subMeta.name.slice(0, -1), ...uniqueKeys];
//       const mergedNameKey = mergedNamePath.join(NAME_SPLIT);

//       if (subMeta.destroy) {
//         // Remove
//         delete clone[mergedNameKey];
//       } else {
//         // Update
//         clone[mergedNameKey] = subMeta;
//       }

//       return clone;
//     });
//   };

//   // >>>>> Get merged errors
//   const [mergedErrors, mergedWarnings] = React.useMemo(() => {
//     const errorList: string[] = [...meta.errors];
//     const warningList: string[] = [...meta.warnings];

//     Object.values(subFieldErrors).forEach(subFieldError => {
//       errorList.push(...(subFieldError.errors || []));
//       warningList.push(...(subFieldError.warnings || []));
//     });

//     return [errorList, warningList];
//   }, [subFieldErrors, meta.errors, meta.warnings]);

//   const debounceErrors = useDebounce(mergedErrors);
//   const debounceWarnings = useDebounce(mergedWarnings);

//   // ===================== Children Ref =====================
//   const getItemRef = useItemRef();

//   // ======================== Status ========================
//   let mergedValidateStatus: ValidateStatus = '';
//   if (validateStatus !== undefined) {
//     mergedValidateStatus = validateStatus;
//   } else if (meta?.validating) {
//     mergedValidateStatus = 'validating';
//   } else if (debounceErrors.length) {
//     mergedValidateStatus = 'error';
//   } else if (debounceWarnings.length) {
//     mergedValidateStatus = 'warning';
//   } else if (meta?.touched) {
//     mergedValidateStatus = 'success';
//   }

//   const formItemStatusContext = useMemo<FormItemStatusContextProps>(() => {
//     let feedbackIcon: ReactNode;
//     if (hasFeedback) {
//       const IconNode = mergedValidateStatus && iconMap[mergedValidateStatus];
//       feedbackIcon = IconNode ? (
//         <span
//           className={classNames(
//             `${prefixCls}-item-feedback-icon`,
//             `${prefixCls}-item-feedback-icon-${mergedValidateStatus}`,
//           )}
//         >
//           <IconNode />
//         </span>
//       ) : null;
//     }

//     return {
//       status: mergedValidateStatus,
//       hasFeedback,
//       feedbackIcon,
//       isFormItemInput: true,
//     };
//   }, [mergedValidateStatus, hasFeedback]);

//   // ======================== Render ========================
//   function renderLayout(
//     baseChildren: React.ReactNode,
//     fieldId?: string,
//     isRequired?: boolean,
//   ): React.ReactNode {
//     if (noStyle && !hidden) {
//       return baseChildren;
//     }

//     const itemClassName = {
//       [`${prefixCls}-item`]: true,
//       [`${prefixCls}-item-with-help`]:
//         (help !== undefined && help !== null) || debounceErrors.length || debounceWarnings.length,
//       [`${className}`]: !!className,

//       // Status
//       [`${prefixCls}-item-has-feedback`]: mergedValidateStatus && hasFeedback,
//       [`${prefixCls}-item-has-success`]: mergedValidateStatus === 'success',
//       [`${prefixCls}-item-has-warning`]: mergedValidateStatus === 'warning',
//       [`${prefixCls}-item-has-error`]: mergedValidateStatus === 'error',
//       [`${prefixCls}-item-is-validating`]: mergedValidateStatus === 'validating',
//       [`${prefixCls}-item-hidden`]: hidden,
//     };

//     // ======================= Children =======================
//     return (
//       <Row
//         className={classNames(itemClassName)}
//         style={style}
//         key="row"
//         {...omit(restProps, [
//           'colon',
//           'extra',
//           'fieldKey',
//           'requiredMark',
//           'getValueFromEvent',
//           'getValueProps',
//           'htmlFor',
//           'id', // It is deprecated because `htmlFor` is its replacement.
//           'initialValue',
//           'isListField',
//           'labelAlign',
//           'labelWrap',
//           'labelCol',
//           'normalize',
//           'preserve',
//           'tooltip',
//           'validateFirst',
//           'valuePropName',
//           'wrapperCol',
//           '_internalItemRender' as any,
//         ])}
//       >
//         {/* Label */}
//         <FormItemLabel
//           htmlFor={fieldId}
//           required={isRequired}
//           requiredMark={requiredMark}
//           {...props}
//           prefixCls={prefixCls}
//         />
//         {/* Input Group */}
//         <FormItemInput
//           {...props}
//           {...meta}
//           errors={debounceErrors}
//           warnings={debounceWarnings}
//           prefixCls={prefixCls}
//           status={mergedValidateStatus}
//           help={help}
//         >
//           <NoStyleItemContext.Provider value={onSubItemMetaChange}>
//             <FormItemInputContext.Provider value={formItemStatusContext}>
//               {baseChildren}
//             </FormItemInputContext.Provider>
//           </NoStyleItemContext.Provider>
//         </FormItemInput>
//       </Row>
//     );
//   }

//   if (!hasName && !isRenderProps && !dependencies) {
//     return renderLayout(children) as JSX.Element;
//   }

//   let variables: Record<string, string> = {};
//   if (typeof label === 'string') {
//     variables.label = label;
//   } else if (name) {
//     variables.label = String(name);
//   }
//   if (messageVariables) {
//     variables = { ...variables, ...messageVariables };
//   }

//   // >>>>> With Field
//   return (
//     <Field
//       {...props}
//       messageVariables={variables}
//       trigger={trigger}
//       validateTrigger={mergedValidateTrigger}
//       onMetaChange={onMetaChange}
//     >
//       {(control, renderMeta, context) => {
//         const mergedName = toArray(name).length && renderMeta ? renderMeta.name : [];
//         const fieldId = getFieldId(mergedName, formName);

//         const isRequired =
//           required !== undefined
//             ? required
//             : !!(
//                 rules &&
//                 rules.some(rule => {
//                   if (rule && typeof rule === 'object' && rule.required && !rule.warningOnly) {
//                     return true;
//                   }
//                   if (typeof rule === 'function') {
//                     const ruleEntity = rule(context);
//                     return ruleEntity && ruleEntity.required && !ruleEntity.warningOnly;
//                   }
//                   return false;
//                 })
//               );

//         // ======================= Children =======================
//         const mergedControl: typeof control = {
//           ...control,
//         };

//         let childNode: React.ReactNode = null;

//         warning(
//           !(shouldUpdate && dependencies),
//           'Form.Item',
//           "`shouldUpdate` and `dependencies` shouldn't be used together. See https://ant.design/components/form/#dependencies.",
//         );
//         if (Array.isArray(children) && hasName) {
//           warning(false, 'Form.Item', '`children` is array of render props cannot have `name`.');
//           childNode = children;
//         } else if (isRenderProps && (!(shouldUpdate || dependencies) || hasName)) {
//           warning(
//             !!(shouldUpdate || dependencies),
//             'Form.Item',
//             '`children` of render props only work with `shouldUpdate` or `dependencies`.',
//           );
//           warning(
//             !hasName,
//             'Form.Item',
//             "Do not use `name` with `children` of render props since it's not a field.",
//           );
//         } else if (dependencies && !isRenderProps && !hasName) {
//           warning(
//             false,
//             'Form.Item',
//             'Must set `name` or use render props when `dependencies` is set.',
//           );
//         } else if (isValidElement(children)) {
//           warning(
//             children.props.defaultValue === undefined,
//             'Form.Item',
//             '`defaultValue` will not work on controlled Field. You should use `initialValues` of Form instead.',
//           );

//           const childProps = { ...children.props, ...mergedControl };
//           if (!childProps.id) {
//             childProps.id = fieldId;
//           }

//           if (supportRef(children)) {
//             childProps.ref = getItemRef(mergedName, children);
//           }

//           // We should keep user origin event handler
//           const triggers = new Set<string>([
//             ...toArray(trigger),
//             ...toArray(mergedValidateTrigger),
//           ]);

//           triggers.forEach(eventName => {
//             childProps[eventName] = (...args: any[]) => {
//               mergedControl[eventName]?.(...args);
//               children.props[eventName]?.(...args);
//             };
//           });

//           childNode = (
//             <MemoInput value={mergedControl[props.valuePropName || 'value']} update={children}>
//               {cloneElement(children, childProps)}
//             </MemoInput>
//           );
//         } else if (isRenderProps && (shouldUpdate || dependencies) && !hasName) {
//           childNode = (children as RenderChildren)(context);
//         } else {
//           warning(
//             !mergedName.length,
//             'Form.Item',
//             '`name` is only used for validate React element. If you are using Form.Item as layout display, please remove `name` instead.',
//           );
//           childNode = children as React.ReactNode;
//         }

//         return renderLayout(childNode, fieldId, isRequired);
//       }}
//     </Field>
//   );
// }

// export default FormItem;

// import classNames from 'classnames';
// import * as React from 'react';
// import type { ColProps } from '../grid/col';
// import Col from '../grid/col';
// import { FormContext, FormItemPrefixContext } from './context';
// import ErrorList from './ErrorList';
// import type { ValidateStatus } from './FormItem';

// interface FormItemInputMiscProps {
//   prefixCls: string;
//   children: React.ReactNode;
//   errors: React.ReactNode[];
//   warnings: React.ReactNode[];
//   /** @private Internal Usage, do not use in any of your production. */
//   _internalItemRender?: {
//     mark: string;
//     render: (
//       props: FormItemInputProps & FormItemInputMiscProps,
//       domList: {
//         input: JSX.Element;
//         errorList: JSX.Element;
//         extra: JSX.Element | null;
//       },
//     ) => React.ReactNode;
//   };
// }

// export interface FormItemInputProps {
//   wrapperCol?: ColProps;
//   extra?: React.ReactNode;
//   status?: ValidateStatus;
//   help?: React.ReactNode;
// }

// const FormItemInput: React.FC<FormItemInputProps & FormItemInputMiscProps> = props => {
//   const {
//     prefixCls,
//     status,
//     wrapperCol,
//     children,
//     errors,
//     warnings,
//     _internalItemRender: formItemRender,
//     extra,
//     help,
//   } = props;
//   const baseClassName = `${prefixCls}-item`;

//   const formContext = React.useContext(FormContext);

//   const mergedWrapperCol: ColProps = wrapperCol || formContext.wrapperCol || {};

//   const className = classNames(`${baseClassName}-control`, mergedWrapperCol.className);

//   // Pass to sub FormItem should not with col info
//   const subFormContext = React.useMemo(() => ({ ...formContext }), [formContext]);
//   delete subFormContext.labelCol;
//   delete subFormContext.wrapperCol;

//   const inputDom = (
//     <div className={`${baseClassName}-control-input`}>
//       <div className={`${baseClassName}-control-input-content`}>{children}</div>
//     </div>
//   );
//   const formItemContext = React.useMemo(() => ({ prefixCls, status }), [prefixCls, status]);
//   const errorListDom = (
//     <FormItemPrefixContext.Provider value={formItemContext}>
//       <ErrorList
//         errors={errors}
//         warnings={warnings}
//         help={help}
//         helpStatus={status}
//         className={`${baseClassName}-explain-connected`}
//       />
//     </FormItemPrefixContext.Provider>
//   );

//   // If extra = 0, && will goes wrong
//   // 0&&error -> 0
//   const extraDom = extra ? <div className={`${baseClassName}-extra`}>{extra}</div> : null;

//   const dom =
//     formItemRender && formItemRender.mark === 'pro_table_render' && formItemRender.render ? (
//       formItemRender.render(props, { input: inputDom, errorList: errorListDom, extra: extraDom })
//     ) : (
//       <>
//         {inputDom}
//         {errorListDom}
//         {extraDom}
//       </>
//     );
//   return (
//     <FormContext.Provider value={subFormContext}>
//       <Col {...mergedWrapperCol} className={className}>
//         {dom}
//       </Col>
//     </FormContext.Provider>
//   );
// };

// export default FormItemInput;

// import QuestionCircleOutlined from '@ant-design/icons/QuestionCircleOutlined';
// import classNames from 'classnames';
// import * as React from 'react';
// import type { ColProps } from '../grid/col';
// import Col from '../grid/col';
// import { useLocaleReceiver } from '../locale-provider/LocaleReceiver';
// import defaultLocale from '../locale/default';
// import type { TooltipProps } from '../tooltip';
// import Tooltip from '../tooltip';
// import type { FormContextProps } from './context';
// import { FormContext } from './context';
// import type { RequiredMark } from './Form';
// import type { FormLabelAlign } from './interface';

// export type WrapperTooltipProps = TooltipProps & {
//   icon?: React.ReactElement;
// };

// export type LabelTooltipType = WrapperTooltipProps | React.ReactNode;

// function toTooltipProps(tooltip: LabelTooltipType): WrapperTooltipProps | null {
//   if (!tooltip) {
//     return null;
//   }

//   if (typeof tooltip === 'object' && !React.isValidElement(tooltip)) {
//     return tooltip as WrapperTooltipProps;
//   }

//   return {
//     title: tooltip,
//   };
// }

// export interface FormItemLabelProps {
//   colon?: boolean;
//   htmlFor?: string;
//   label?: React.ReactNode;
//   labelAlign?: FormLabelAlign;
//   labelCol?: ColProps;
//   requiredMark?: RequiredMark;
//   tooltip?: LabelTooltipType;
// }

// const FormItemLabel: React.FC<FormItemLabelProps & { required?: boolean; prefixCls: string }> = ({
//   prefixCls,
//   label,
//   htmlFor,
//   labelCol,
//   labelAlign,
//   colon,
//   required,
//   requiredMark,
//   tooltip,
// }) => {
//   const [formLocale] = useLocaleReceiver('Form');

//   if (!label) return null;

//   return (
//     <FormContext.Consumer key="label">
//       {({
//         vertical,
//         labelAlign: contextLabelAlign,
//         labelCol: contextLabelCol,
//         labelWrap,
//         colon: contextColon,
//       }: FormContextProps) => {
//         const mergedLabelCol: ColProps = labelCol || contextLabelCol || {};

//         const mergedLabelAlign: FormLabelAlign | undefined = labelAlign || contextLabelAlign;

//         const labelClsBasic = `${prefixCls}-item-label`;
//         const labelColClassName = classNames(
//           labelClsBasic,
//           mergedLabelAlign === 'left' && `${labelClsBasic}-left`,
//           mergedLabelCol.className,
//           {
//             [`${labelClsBasic}-wrap`]: !!labelWrap,
//           },
//         );

//         let labelChildren = label;
//         // Keep label is original where there should have no colon
//         const computedColon = colon === true || (contextColon !== false && colon !== false);
//         const haveColon = computedColon && !vertical;
//         // Remove duplicated user input colon
//         if (haveColon && typeof label === 'string' && (label as string).trim() !== '') {
//           labelChildren = (label as string).replace(/[:|：]\s*$/, '');
//         }

//         // Tooltip
//         const tooltipProps = toTooltipProps(tooltip);
//         if (tooltipProps) {
//           const { icon = <QuestionCircleOutlined />, ...restTooltipProps } = tooltipProps;
//           const tooltipNode = (
//             <Tooltip {...restTooltipProps}>
//               {React.cloneElement(icon, { className: `${prefixCls}-item-tooltip`, title: '' })}
//             </Tooltip>
//           );

//           labelChildren = (
//             <>
//               {labelChildren}
//               {tooltipNode}
//             </>
//           );
//         }

//         // Add required mark if optional
//         if (requiredMark === 'optional' && !required) {
//           labelChildren = (
//             <>
//               {labelChildren}
//               <span className={`${prefixCls}-item-optional`} title="">
//                 {formLocale?.optional || defaultLocale.Form?.optional}
//               </span>
//             </>
//           );
//         }

//         const labelClassName = classNames({
//           [`${prefixCls}-item-required`]: required,
//           [`${prefixCls}-item-required-mark-optional`]: requiredMark === 'optional',
//           [`${prefixCls}-item-no-colon`]: !computedColon,
//         });

//         return (
//           <Col {...mergedLabelCol} className={labelColClassName}>
//             <label
//               htmlFor={htmlFor}
//               className={labelClassName}
//               title={typeof label === 'string' ? label : ''}
//             >
//               {labelChildren}
//             </label>
//           </Col>
//         );
//       }}
//     </FormContext.Consumer>
//   );
// };

// export default FormItemLabel;

// import { List } from 'rc-field-form';
// import type { StoreValue, ValidatorRule } from 'rc-field-form/lib/interface';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import warning from '../_util/warning';
// import { FormItemPrefixContext } from './context';

// export interface FormListFieldData {
//   name: number;
//   key: number;
//   /** @deprecated No need anymore Use key instead */
//   fieldKey?: number;
// }

// export interface FormListOperation {
//   add: (defaultValue?: StoreValue, insertIndex?: number) => void;
//   remove: (index: number | number[]) => void;
//   move: (from: number, to: number) => void;
// }

// export interface FormListProps {
//   prefixCls?: string;
//   name: string | number | (string | number)[];
//   rules?: ValidatorRule[];
//   initialValue?: any[];
//   children: (
//     fields: FormListFieldData[],
//     operation: FormListOperation,
//     meta: { errors: React.ReactNode[]; warnings: React.ReactNode[] },
//   ) => React.ReactNode;
// }

// const FormList: React.FC<FormListProps> = ({
//   prefixCls: customizePrefixCls,
//   children,
//   ...props
// }) => {
//   warning(!!props.name, 'Form.List', 'Miss `name` prop.');

//   const { getPrefixCls } = React.useContext(ConfigContext);
//   const prefixCls = getPrefixCls('form', customizePrefixCls);
//   const contextValue = React.useMemo(
//     () => ({
//       prefixCls,
//       status: 'error' as const,
//     }),
//     [prefixCls],
//   );

//   return (
//     <List {...props}>
//       {(fields, operation, meta) => (
//         <FormItemPrefixContext.Provider value={contextValue}>
//           {children(
//             fields.map(field => ({ ...field, fieldKey: field.key })),
//             operation,
//             {
//               errors: meta.errors,
//               warnings: meta.warnings,
//             },
//           )}
//         </FormItemPrefixContext.Provider>
//       )}
//     </List>
//   );
// };

// export default FormList;

// import { Rule, RuleObject, RuleRender } from 'rc-field-form/lib/interface';
// import warning from '../_util/warning';
// import { FormProvider } from './context';
// import ErrorList, { ErrorListProps } from './ErrorList';
// import InternalForm, { FormInstance, FormProps, useForm, useWatch } from './Form';
// import Item, { FormItemProps } from './FormItem';
// import List, { FormListFieldData, FormListOperation, FormListProps } from './FormList';
// import useFormInstance from './hooks/useFormInstance';

// type InternalFormType = typeof InternalForm;

// interface FormInterface extends InternalFormType {
//   useForm: typeof useForm;
//   useFormInstance: typeof useFormInstance;
//   useWatch: typeof useWatch;
//   Item: typeof Item;
//   List: typeof List;
//   ErrorList: typeof ErrorList;
//   Provider: typeof FormProvider;

//   /** @deprecated Only for warning usage. Do not use. */
//   create: () => void;
// }

// const Form = InternalForm as FormInterface;

// Form.Item = Item;
// Form.List = List;
// Form.ErrorList = ErrorList;
// Form.useForm = useForm;
// Form.useFormInstance = useFormInstance;
// Form.useWatch = useWatch;
// Form.Provider = FormProvider;
// Form.create = () => {
//   warning(
//     false,
//     'Form',
//     'antd v4 removed `Form.create`. Please remove or use `@ant-design/compatible` instead.',
//   );
// };

// export {
//   FormInstance,
//   FormProps,
//   FormItemProps,
//   ErrorListProps,
//   Rule,
//   RuleObject,
//   RuleRender,
//   FormListProps,
//   FormListFieldData,
//   FormListOperation,
// };

// export default Form;

// export { InternalNamePath, NamePath, Store, StoreValue } from 'rc-field-form/lib/interface';
// export { Options as ScrollOptions } from 'scroll-into-view-if-needed';
// export type FormLabelAlign = 'left' | 'right';

// import type { InternalNamePath } from './interface';

// // form item name black list.  in form ,you can use form.id get the form item element.
// // use object hasOwnProperty will get better performance if black list is longer.
// const formItemNameBlackList = ['parentNode'];

// // default form item id prefix.
// const defaultItemNamePrefixCls: string = 'form_item';

// export function toArray<T>(candidate?: T | T[] | false): T[] {
//   if (candidate === undefined || candidate === false) return [];

//   return Array.isArray(candidate) ? candidate : [candidate];
// }

// export function getFieldId(namePath: InternalNamePath, formName?: string): string | undefined {
//   if (!namePath.length) return undefined;

//   const mergedId = namePath.join('_');

//   if (formName) {
//     return `${formName}_${mergedId}`;
//   }

//   const isIllegalName = formItemNameBlackList.indexOf(mergedId) >= 0;

//   return isIllegalName ? `${defaultItemNamePrefixCls}_${mergedId}` : mergedId;
// }
