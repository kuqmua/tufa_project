// import classNames from 'classnames';
// import * as React from 'react';
// import ConfigProvider from '../config-provider';
// import ActionButton from '../_util/ActionButton';
// import { getTransitionName } from '../_util/motion';
// import warning from '../_util/warning';
// import type { ModalFuncProps } from './Modal';
// import Dialog from './Modal';

// interface ConfirmDialogProps extends ModalFuncProps {
//   afterClose?: () => void;
//   close: (...args: any[]) => void;
//   autoFocusButton?: null | 'ok' | 'cancel';
//   rootPrefixCls: string;
//   iconPrefixCls?: string;
// }

// const ConfirmDialog = (props: ConfirmDialogProps) => {
//   const {
//     icon,
//     onCancel,
//     onOk,
//     close,
//     zIndex,
//     afterClose,
//     visible,
//     keyboard,
//     centered,
//     getContainer,
//     maskStyle,
//     okText,
//     okButtonProps,
//     cancelText,
//     cancelButtonProps,
//     direction,
//     prefixCls,
//     wrapClassName,
//     rootPrefixCls,
//     iconPrefixCls,
//     bodyStyle,
//     closable = false,
//     closeIcon,
//     modalRender,
//     focusTriggerAfterClose,
//   } = props;

//   warning(
//     !(typeof icon === 'string' && icon.length > 2),
//     'Modal',
//     `\`icon\` is using ReactNode instead of string naming in v4. Please check \`${icon}\` at https://ant.design/components/icon`,
//   );

//   // 支持传入{ icon: null }来隐藏`Modal.confirm`默认的Icon
//   const okType = props.okType || 'primary';
//   const contentPrefixCls = `${prefixCls}-confirm`;
//   // 默认为 true，保持向下兼容
//   const okCancel = 'okCancel' in props ? props.okCancel! : true;
//   const width = props.width || 416;
//   const style = props.style || {};
//   const mask = props.mask === undefined ? true : props.mask;
//   // 默认为 false，保持旧版默认行为
//   const maskClosable = props.maskClosable === undefined ? false : props.maskClosable;
//   const autoFocusButton = props.autoFocusButton === null ? false : props.autoFocusButton || 'ok';

//   const classString = classNames(
//     contentPrefixCls,
//     `${contentPrefixCls}-${props.type}`,
//     { [`${contentPrefixCls}-rtl`]: direction === 'rtl' },
//     props.className,
//   );

//   const cancelButton = okCancel && (
//     <ActionButton
//       actionFn={onCancel}
//       close={close}
//       autoFocus={autoFocusButton === 'cancel'}
//       buttonProps={cancelButtonProps}
//       prefixCls={`${rootPrefixCls}-btn`}
//     >
//       {cancelText}
//     </ActionButton>
//   );

//   return (
//     <ConfigProvider prefixCls={rootPrefixCls} iconPrefixCls={iconPrefixCls} direction={direction}>
//       <Dialog
//         prefixCls={prefixCls}
//         className={classString}
//         wrapClassName={classNames(
//           { [`${contentPrefixCls}-centered`]: !!props.centered },
//           wrapClassName,
//         )}
//         onCancel={() => close({ triggerCancel: true })}
//         visible={visible}
//         title=""
//         footer=""
//         transitionName={getTransitionName(rootPrefixCls, 'zoom', props.transitionName)}
//         maskTransitionName={getTransitionName(rootPrefixCls, 'fade', props.maskTransitionName)}
//         mask={mask}
//         maskClosable={maskClosable}
//         maskStyle={maskStyle}
//         style={style}
//         bodyStyle={bodyStyle}
//         width={width}
//         zIndex={zIndex}
//         afterClose={afterClose}
//         keyboard={keyboard}
//         centered={centered}
//         getContainer={getContainer}
//         closable={closable}
//         closeIcon={closeIcon}
//         modalRender={modalRender}
//         focusTriggerAfterClose={focusTriggerAfterClose}
//       >
//         <div className={`${contentPrefixCls}-body-wrapper`}>
//           <div className={`${contentPrefixCls}-body`}>
//             {icon}
//             {props.title === undefined ? null : (
//               <span className={`${contentPrefixCls}-title`}>{props.title}</span>
//             )}
//             <div className={`${contentPrefixCls}-content`}>{props.content}</div>
//           </div>
//           <div className={`${contentPrefixCls}-btns`}>
//             {cancelButton}
//             <ActionButton
//               type={okType}
//               actionFn={onOk}
//               close={close}
//               autoFocus={autoFocusButton === 'ok'}
//               buttonProps={okButtonProps}
//               prefixCls={`${rootPrefixCls}-btn`}
//             >
//               {okText}
//             </ActionButton>
//           </div>
//         </div>
//       </Dialog>
//     </ConfigProvider>
//   );
// };

// export default ConfirmDialog;

// import CloseOutlined from '@ant-design/icons/CloseOutlined';
// import classNames from 'classnames';
// import Dialog from 'rc-dialog';
// import * as React from 'react';

// import Button from '../button';
// import type { ButtonProps, LegacyButtonType } from '../button/button';
// import { convertLegacyProps } from '../button/button';
// import type { DirectionType } from '../config-provider';
// import { ConfigContext } from '../config-provider';
// import { NoFormStyle } from '../form/context';
// import LocaleReceiver from '../locale-provider/LocaleReceiver';
// import { getTransitionName } from '../_util/motion';
// import { canUseDocElement } from '../_util/styleChecker';
// import { getConfirmLocale } from './locale';

// let mousePosition: { x: number; y: number } | null;

// // ref: https://github.com/ant-design/ant-design/issues/15795
// const getClickPosition = (e: MouseEvent) => {
//   mousePosition = {
//     x: e.pageX,
//     y: e.pageY,
//   };
//   // 100ms 内发生过点击事件，则从点击位置动画展示
//   // 否则直接 zoom 展示
//   // 这样可以兼容非点击方式展开
//   setTimeout(() => {
//     mousePosition = null;
//   }, 100);
// };

// // 只有点击事件支持从鼠标位置动画展开
// if (canUseDocElement()) {
//   document.documentElement.addEventListener('click', getClickPosition, true);
// }

// export interface ModalProps {
//   /** 对话框是否可见 */
//   visible?: boolean;
//   /** 确定按钮 loading */
//   confirmLoading?: boolean;
//   /** 标题 */
//   title?: React.ReactNode;
//   /** 是否显示右上角的关闭按钮 */
//   closable?: boolean;
//   /** 点击确定回调 */
//   onOk?: (e: React.MouseEvent<HTMLElement>) => void;
//   /** 点击模态框右上角叉、取消按钮、Props.maskClosable 值为 true 时的遮罩层或键盘按下 Esc 时的回调 */
//   onCancel?: (e: React.MouseEvent<HTMLElement>) => void;
//   afterClose?: () => void;
//   /** 垂直居中 */
//   centered?: boolean;
//   /** 宽度 */
//   width?: string | number;
//   /** 底部内容 */
//   footer?: React.ReactNode;
//   /** 确认按钮文字 */
//   okText?: React.ReactNode;
//   /** 确认按钮类型 */
//   okType?: LegacyButtonType;
//   /** 取消按钮文字 */
//   cancelText?: React.ReactNode;
//   /** 点击蒙层是否允许关闭 */
//   maskClosable?: boolean;
//   /** 强制渲染 Modal */
//   forceRender?: boolean;
//   okButtonProps?: ButtonProps;
//   cancelButtonProps?: ButtonProps;
//   destroyOnClose?: boolean;
//   style?: React.CSSProperties;
//   wrapClassName?: string;
//   maskTransitionName?: string;
//   transitionName?: string;
//   className?: string;
//   getContainer?: string | HTMLElement | getContainerFunc | false;
//   zIndex?: number;
//   bodyStyle?: React.CSSProperties;
//   maskStyle?: React.CSSProperties;
//   mask?: boolean;
//   keyboard?: boolean;
//   wrapProps?: any;
//   prefixCls?: string;
//   closeIcon?: React.ReactNode;
//   modalRender?: (node: React.ReactNode) => React.ReactNode;
//   focusTriggerAfterClose?: boolean;
//   children?: React.ReactNode;
// }

// type getContainerFunc = () => HTMLElement;

// export interface ModalFuncProps {
//   prefixCls?: string;
//   className?: string;
//   visible?: boolean;
//   title?: React.ReactNode;
//   closable?: boolean;
//   content?: React.ReactNode;
//   // TODO: find out exact types
//   onOk?: (...args: any[]) => any;
//   onCancel?: (...args: any[]) => any;
//   afterClose?: () => void;
//   okButtonProps?: ButtonProps;
//   cancelButtonProps?: ButtonProps;
//   centered?: boolean;
//   width?: string | number;
//   okText?: React.ReactNode;
//   okType?: LegacyButtonType;
//   cancelText?: React.ReactNode;
//   icon?: React.ReactNode;
//   mask?: boolean;
//   maskClosable?: boolean;
//   zIndex?: number;
//   okCancel?: boolean;
//   style?: React.CSSProperties;
//   wrapClassName?: string;
//   maskStyle?: React.CSSProperties;
//   type?: 'info' | 'success' | 'error' | 'warn' | 'warning' | 'confirm';
//   keyboard?: boolean;
//   getContainer?: string | HTMLElement | getContainerFunc | false;
//   autoFocusButton?: null | 'ok' | 'cancel';
//   transitionName?: string;
//   maskTransitionName?: string;
//   direction?: DirectionType;
//   bodyStyle?: React.CSSProperties;
//   closeIcon?: React.ReactNode;
//   modalRender?: (node: React.ReactNode) => React.ReactNode;
//   focusTriggerAfterClose?: boolean;
// }

// export interface ModalLocale {
//   okText: string;
//   cancelText: string;
//   justOkText: string;
// }

// const Modal: React.FC<ModalProps> = props => {
//   const {
//     getPopupContainer: getContextPopupContainer,
//     getPrefixCls,
//     direction,
//   } = React.useContext(ConfigContext);

//   const handleCancel = (e: React.MouseEvent<HTMLButtonElement>) => {
//     const { onCancel } = props;
//     onCancel?.(e);
//   };

//   const handleOk = (e: React.MouseEvent<HTMLButtonElement>) => {
//     const { onOk } = props;
//     onOk?.(e);
//   };

//   const renderFooter = (locale: ModalLocale) => {
//     const { okText, okType, cancelText, confirmLoading } = props;
//     return (
//       <>
//         <Button onClick={handleCancel} {...props.cancelButtonProps}>
//           {cancelText || locale.cancelText}
//         </Button>
//         <Button
//           {...convertLegacyProps(okType)}
//           loading={confirmLoading}
//           onClick={handleOk}
//           {...props.okButtonProps}
//         >
//           {okText || locale.okText}
//         </Button>
//       </>
//     );
//   };

//   const {
//     prefixCls: customizePrefixCls,
//     footer,
//     visible,
//     wrapClassName,
//     centered,
//     getContainer,
//     closeIcon,
//     focusTriggerAfterClose = true,
//     ...restProps
//   } = props;

//   const prefixCls = getPrefixCls('modal', customizePrefixCls);
//   const rootPrefixCls = getPrefixCls();

//   const defaultFooter = (
//     <LocaleReceiver componentName="Modal" defaultLocale={getConfirmLocale()}>
//       {renderFooter}
//     </LocaleReceiver>
//   );

//   const closeIconToRender = (
//     <span className={`${prefixCls}-close-x`}>
//       {closeIcon || <CloseOutlined className={`${prefixCls}-close-icon`} />}
//     </span>
//   );

//   const wrapClassNameExtended = classNames(wrapClassName, {
//     [`${prefixCls}-centered`]: !!centered,
//     [`${prefixCls}-wrap-rtl`]: direction === 'rtl',
//   });
//   return (
//     <NoFormStyle status override>
//       <Dialog
//         {...restProps}
//         getContainer={
//           getContainer === undefined ? (getContextPopupContainer as getContainerFunc) : getContainer
//         }
//         prefixCls={prefixCls}
//         wrapClassName={wrapClassNameExtended}
//         footer={footer === undefined ? defaultFooter : footer}
//         visible={visible}
//         mousePosition={mousePosition}
//         onClose={handleCancel}
//         closeIcon={closeIconToRender}
//         focusTriggerAfterClose={focusTriggerAfterClose}
//         transitionName={getTransitionName(rootPrefixCls, 'zoom', props.transitionName)}
//         maskTransitionName={getTransitionName(rootPrefixCls, 'fade', props.maskTransitionName)}
//       />
//     </NoFormStyle>
//   );
// };

// Modal.defaultProps = {
//   width: 520,
//   confirmLoading: false,
//   visible: false,
//   okType: 'primary' as LegacyButtonType,
// };

// export default Modal;

// import CheckCircleOutlined from '@ant-design/icons/CheckCircleOutlined';
// import CloseCircleOutlined from '@ant-design/icons/CloseCircleOutlined';
// import ExclamationCircleOutlined from '@ant-design/icons/ExclamationCircleOutlined';
// import InfoCircleOutlined from '@ant-design/icons/InfoCircleOutlined';
// import { render as reactRender, unmount as reactUnmount } from 'rc-util/lib/React/render';
// import * as React from 'react';
// import { globalConfig } from '../config-provider';
// import warning from '../_util/warning';
// import ConfirmDialog from './ConfirmDialog';
// import destroyFns from './destroyFns';
// import { getConfirmLocale } from './locale';
// import type { ModalFuncProps } from './Modal';

// let defaultRootPrefixCls = '';

// function getRootPrefixCls() {
//   return defaultRootPrefixCls;
// }

// type ConfigUpdate = ModalFuncProps | ((prevConfig: ModalFuncProps) => ModalFuncProps);

// export type ModalFunc = (props: ModalFuncProps) => {
//   destroy: () => void;
//   update: (configUpdate: ConfigUpdate) => void;
// };

// export type ModalStaticFunctions = Record<NonNullable<ModalFuncProps['type']>, ModalFunc>;

// export default function confirm(config: ModalFuncProps) {
//   const container = document.createDocumentFragment();
//   // eslint-disable-next-line @typescript-eslint/no-use-before-define
//   let currentConfig = { ...config, close, visible: true } as any;

//   function destroy(...args: any[]) {
//     const triggerCancel = args.some(param => param && param.triggerCancel);
//     if (config.onCancel && triggerCancel) {
//       config.onCancel(...args);
//     }
//     for (let i = 0; i < destroyFns.length; i++) {
//       const fn = destroyFns[i];
//       // eslint-disable-next-line @typescript-eslint/no-use-before-define
//       if (fn === close) {
//         destroyFns.splice(i, 1);
//         break;
//       }
//     }

//     reactUnmount(container);
//   }

//   function render({ okText, cancelText, prefixCls: customizePrefixCls, ...props }: any) {
//     /**
//      * https://github.com/ant-design/ant-design/issues/23623
//      *
//      * Sync render blocks React event. Let's make this async.
//      */
//     setTimeout(() => {
//       const runtimeLocale = getConfirmLocale();
//       const { getPrefixCls, getIconPrefixCls } = globalConfig();
//       // because Modal.config  set rootPrefixCls, which is different from other components
//       const rootPrefixCls = getPrefixCls(undefined, getRootPrefixCls());
//       const prefixCls = customizePrefixCls || `${rootPrefixCls}-modal`;
//       const iconPrefixCls = getIconPrefixCls();

//       reactRender(
//         <ConfirmDialog
//           {...props}
//           prefixCls={prefixCls}
//           rootPrefixCls={rootPrefixCls}
//           iconPrefixCls={iconPrefixCls}
//           okText={okText || (props.okCancel ? runtimeLocale.okText : runtimeLocale.justOkText)}
//           cancelText={cancelText || runtimeLocale.cancelText}
//         />,
//         container,
//       );
//     });
//   }

//   function close(...args: any[]) {
//     currentConfig = {
//       ...currentConfig,
//       visible: false,
//       afterClose: () => {
//         if (typeof config.afterClose === 'function') {
//           config.afterClose();
//         }

//         destroy.apply(this, args);
//       },
//     };
//     render(currentConfig);
//   }

//   function update(configUpdate: ConfigUpdate) {
//     if (typeof configUpdate === 'function') {
//       currentConfig = configUpdate(currentConfig);
//     } else {
//       currentConfig = {
//         ...currentConfig,
//         ...configUpdate,
//       };
//     }
//     render(currentConfig);
//   }

//   render(currentConfig);

//   destroyFns.push(close);

//   return {
//     destroy: close,
//     update,
//   };
// }

// export function withWarn(props: ModalFuncProps): ModalFuncProps {
//   return {
//     icon: <ExclamationCircleOutlined />,
//     okCancel: false,
//     ...props,
//     type: 'warning',
//   };
// }

// export function withInfo(props: ModalFuncProps): ModalFuncProps {
//   return {
//     icon: <InfoCircleOutlined />,
//     okCancel: false,
//     ...props,
//     type: 'info',
//   };
// }

// export function withSuccess(props: ModalFuncProps): ModalFuncProps {
//   return {
//     icon: <CheckCircleOutlined />,
//     okCancel: false,
//     ...props,
//     type: 'success',
//   };
// }

// export function withError(props: ModalFuncProps): ModalFuncProps {
//   return {
//     icon: <CloseCircleOutlined />,
//     okCancel: false,
//     ...props,
//     type: 'error',
//   };
// }

// export function withConfirm(props: ModalFuncProps): ModalFuncProps {
//   return {
//     icon: <ExclamationCircleOutlined />,
//     okCancel: true,
//     ...props,
//     type: 'confirm',
//   };
// }

// export function modalGlobalConfig({ rootPrefixCls }: { rootPrefixCls: string }) {
//   warning(false, 'Modal', 'Modal.config is deprecated. Please use ConfigProvider.config instead.');
//   defaultRootPrefixCls = rootPrefixCls;
// }

// const destroyFns: Array<() => void> = [];
// export default destroyFns;

// import type { ModalStaticFunctions } from './confirm';
// import confirm, {
//   modalGlobalConfig,
//   withConfirm,
//   withError,
//   withInfo,
//   withSuccess,
//   withWarn,
// } from './confirm';
// import destroyFns from './destroyFns';
// import type { ModalFuncProps } from './Modal';
// import OriginModal from './Modal';
// import useModal from './useModal';

// export { ModalFuncProps, ModalProps } from './Modal';

// function modalWarn(props: ModalFuncProps) {
//   return confirm(withWarn(props));
// }

// type ModalType = typeof OriginModal &
//   ModalStaticFunctions & {
//     useModal: typeof useModal;
//     destroyAll: () => void;
//     config: typeof modalGlobalConfig;
//   };

// const Modal = OriginModal as ModalType;

// Modal.useModal = useModal;

// Modal.info = function infoFn(props: ModalFuncProps) {
//   return confirm(withInfo(props));
// };

// Modal.success = function successFn(props: ModalFuncProps) {
//   return confirm(withSuccess(props));
// };

// Modal.error = function errorFn(props: ModalFuncProps) {
//   return confirm(withError(props));
// };

// Modal.warning = modalWarn;

// Modal.warn = modalWarn;

// Modal.confirm = function confirmFn(props: ModalFuncProps) {
//   return confirm(withConfirm(props));
// };

// Modal.destroyAll = function destroyAllFn() {
//   while (destroyFns.length) {
//     const close = destroyFns.pop();
//     if (close) {
//       close();
//     }
//   }
// };

// Modal.config = modalGlobalConfig;

// export default Modal;

// import defaultLocale from '../locale/default';

// export interface ModalLocale {
//   okText: string;
//   cancelText: string;
//   justOkText: string;
// }

// let runtimeLocale: ModalLocale = {
//   ...(defaultLocale.Modal as ModalLocale),
// };

// export function changeConfirmLocale(newLocale?: ModalLocale) {
//   if (newLocale) {
//     runtimeLocale = {
//       ...runtimeLocale,
//       ...newLocale,
//     };
//   } else {
//     runtimeLocale = {
//       ...(defaultLocale.Modal as ModalLocale),
//     };
//   }
// }

// export function getConfirmLocale() {
//   return runtimeLocale;
// }
