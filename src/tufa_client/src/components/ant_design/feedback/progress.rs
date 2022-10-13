// import { presetPrimaryColors } from '@ant-design/colors';
// import classNames from 'classnames';
// import { Circle as RCCircle } from 'rc-progress';
// import * as React from 'react';
// import type { ProgressGradient, ProgressProps } from './progress';
// import { getSuccessPercent, validProgress } from './utils';

// interface CircleProps extends ProgressProps {
//   prefixCls: string;
//   children: React.ReactNode;
//   progressStatus: string;
//   strokeColor?: string | ProgressGradient;
// }

// function getPercentage({ percent, success, successPercent }: CircleProps) {
//   const realSuccessPercent = validProgress(getSuccessPercent({ success, successPercent }));
//   return [realSuccessPercent, validProgress(validProgress(percent) - realSuccessPercent)];
// }

// function getStrokeColor({
//   success = {},
//   strokeColor,
// }: Partial<CircleProps>): (string | Record<string, string>)[] {
//   const { strokeColor: successColor } = success;
//   return [successColor || presetPrimaryColors.green, strokeColor || null!];
// }

// const Circle: React.FC<CircleProps> = props => {
//   const {
//     prefixCls,
//     width,
//     strokeWidth,
//     trailColor = null as any,
//     strokeLinecap = 'round',
//     gapPosition,
//     gapDegree,
//     type,
//     children,
//     success,
//   } = props;
//   const circleSize = width || 120;
//   const circleStyle = {
//     width: circleSize,
//     height: circleSize,
//     fontSize: circleSize * 0.15 + 6,
//   } as React.CSSProperties;
//   const circleWidth = strokeWidth || 6;
//   const gapPos = gapPosition || (type === 'dashboard' && 'bottom') || undefined;

//   const getGapDegree = () => {
//     // Support gapDeg = 0 when type = 'dashboard'
//     if (gapDegree || gapDegree === 0) {
//       return gapDegree;
//     }
//     if (type === 'dashboard') {
//       return 75;
//     }
//     return undefined;
//   };

//   // using className to style stroke color
//   const isGradient = Object.prototype.toString.call(props.strokeColor) === '[object Object]';
//   const strokeColor = getStrokeColor({ success, strokeColor: props.strokeColor });

//   const wrapperClassName = classNames(`${prefixCls}-inner`, {
//     [`${prefixCls}-circle-gradient`]: isGradient,
//   });

//   return (
//     <div className={wrapperClassName} style={circleStyle}>
//       <RCCircle
//         percent={getPercentage(props)}
//         strokeWidth={circleWidth}
//         trailWidth={circleWidth}
//         strokeColor={strokeColor}
//         strokeLinecap={strokeLinecap}
//         trailColor={trailColor}
//         prefixCls={prefixCls}
//         gapDegree={getGapDegree()}
//         gapPosition={gapPos}
//       />
//       {children}
//     </div>
//   );
// };

// export default Circle;

// import { presetPrimaryColors } from '@ant-design/colors';
// import * as React from 'react';
// import type { DirectionType } from '../config-provider';
// import type { ProgressGradient, ProgressProps, StringGradients } from './progress';
// import { getSuccessPercent, validProgress } from './utils';

// interface LineProps extends ProgressProps {
//   prefixCls: string;
//   direction?: DirectionType;
//   children: React.ReactNode;
//   strokeColor?: string | ProgressGradient;
// }

// /**
//  * @example
//  *   {
//  *     "0%": "#afc163",
//  *     "75%": "#009900",
//  *     "50%": "green", // ====> '#afc163 0%, #66FF00 25%, #00CC00 50%, #009900 75%, #ffffff 100%'
//  *     "25%": "#66FF00",
//  *     "100%": "#ffffff"
//  *   }
//  */
// export const sortGradient = (gradients: StringGradients) => {
//   let tempArr: any[] = [];
//   Object.keys(gradients).forEach(key => {
//     const formattedKey = parseFloat(key.replace(/%/g, ''));
//     if (!isNaN(formattedKey)) {
//       tempArr.push({
//         key: formattedKey,
//         value: gradients[key],
//       });
//     }
//   });
//   tempArr = tempArr.sort((a, b) => a.key - b.key);
//   return tempArr.map(({ key, value }) => `${value} ${key}%`).join(', ');
// };

// /**
//  * Then this man came to realize the truth: Besides six pence, there is the moon. Besides bread and
//  * butter, there is the bug. And... Besides women, there is the code.
//  *
//  * @example
//  *   {
//  *     "0%": "#afc163",
//  *     "25%": "#66FF00",
//  *     "50%": "#00CC00", // ====>  linear-gradient(to right, #afc163 0%, #66FF00 25%,
//  *     "75%": "#009900", //        #00CC00 50%, #009900 75%, #ffffff 100%)
//  *     "100%": "#ffffff"
//  *   }
//  */
// export const handleGradient = (strokeColor: ProgressGradient, directionConfig: DirectionType) => {
//   const {
//     from = presetPrimaryColors.blue,
//     to = presetPrimaryColors.blue,
//     direction = directionConfig === 'rtl' ? 'to left' : 'to right',
//     ...rest
//   } = strokeColor;
//   if (Object.keys(rest).length !== 0) {
//     const sortedGradients = sortGradient(rest as StringGradients);
//     return { backgroundImage: `linear-gradient(${direction}, ${sortedGradients})` };
//   }
//   return { backgroundImage: `linear-gradient(${direction}, ${from}, ${to})` };
// };

// const Line: React.FC<LineProps> = props => {
//   const {
//     prefixCls,
//     direction: directionConfig,
//     percent,
//     strokeWidth,
//     size,
//     strokeColor,
//     strokeLinecap = 'round',
//     children,
//     trailColor = null,
//     success,
//   } = props;

//   const backgroundProps =
//     strokeColor && typeof strokeColor !== 'string'
//       ? handleGradient(strokeColor, directionConfig)
//       : {
//           background: strokeColor,
//         };

//   const borderRadius = strokeLinecap === 'square' || strokeLinecap === 'butt' ? 0 : undefined;
//   const trailStyle = {
//     backgroundColor: trailColor || undefined,
//     borderRadius,
//   };

//   const percentStyle = {
//     width: `${validProgress(percent)}%`,
//     height: strokeWidth || (size === 'small' ? 6 : 8),
//     borderRadius,
//     ...backgroundProps,
//   };

//   const successPercent = getSuccessPercent(props);

//   const successPercentStyle = {
//     width: `${validProgress(successPercent)}%`,
//     height: strokeWidth || (size === 'small' ? 6 : 8),
//     borderRadius,
//     backgroundColor: success?.strokeColor,
//   };

//   const successSegment =
//     successPercent !== undefined ? (
//       <div className={`${prefixCls}-success-bg`} style={successPercentStyle} />
//     ) : null;

//   return (
//     <>
//       <div className={`${prefixCls}-outer`}>
//         <div className={`${prefixCls}-inner`} style={trailStyle}>
//           <div className={`${prefixCls}-bg`} style={percentStyle} />
//           {successSegment}
//         </div>
//       </div>
//       {children}
//     </>
//   );
// };

// export default Line;

// import classNames from 'classnames';
// import * as React from 'react';
// import type { ProgressProps, ProgressSize } from './progress';

// interface ProgressStepsProps extends ProgressProps {
//   steps: number;
//   size?: ProgressSize;
//   strokeColor?: string | string[];
//   trailColor?: string;
// }

// const Steps: React.FC<ProgressStepsProps> = props => {
//   const {
//     size,
//     steps,
//     percent = 0,
//     strokeWidth = 8,
//     strokeColor,
//     trailColor = null as any,
//     prefixCls,
//     children,
//   } = props;
//   const current = Math.round(steps * (percent / 100));
//   const stepWidth = size === 'small' ? 2 : 14;
//   const styledSteps: React.ReactNode[] = new Array(steps);
//   for (let i = 0; i < steps; i++) {
//     const color = Array.isArray(strokeColor) ? strokeColor[i] : strokeColor;
//     styledSteps[i] = (
//       <div
//         key={i}
//         className={classNames(`${prefixCls}-steps-item`, {
//           [`${prefixCls}-steps-item-active`]: i <= current - 1,
//         })}
//         style={{
//           backgroundColor: i <= current - 1 ? color : trailColor,
//           width: stepWidth,
//           height: strokeWidth,
//         }}
//       />
//     );
//   }
//   return (
//     <div className={`${prefixCls}-steps-outer`}>
//       {styledSteps}
//       {children}
//     </div>
//   );
// };

// export default Steps;

// import Progress from './progress';

// export { ProgressProps } from './progress';

// export default Progress;

// import CheckCircleFilled from '@ant-design/icons/CheckCircleFilled';
// import CheckOutlined from '@ant-design/icons/CheckOutlined';
// import CloseCircleFilled from '@ant-design/icons/CloseCircleFilled';
// import CloseOutlined from '@ant-design/icons/CloseOutlined';
// import classNames from 'classnames';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import { tuple } from '../_util/type';
// import warning from '../_util/warning';
// import Circle from './Circle';
// import Line from './Line';
// import Steps from './Steps';
// import { getSuccessPercent, validProgress } from './utils';

// const ProgressTypes = tuple('line', 'circle', 'dashboard');
// export type ProgressType = typeof ProgressTypes[number];
// const ProgressStatuses = tuple('normal', 'exception', 'active', 'success');
// export type ProgressSize = 'default' | 'small';
// export type StringGradients = { [percentage: string]: string };
// type FromToGradients = { from: string; to: string };
// export type ProgressGradient = { direction?: string } & (StringGradients | FromToGradients);

// export interface SuccessProps {
//   percent?: number;
//   /** @deprecated Use `percent` instead */
//   progress?: number;
//   strokeColor?: string;
// }

// export interface ProgressProps {
//   prefixCls?: string;
//   className?: string;
//   type?: ProgressType;
//   percent?: number;
//   format?: (percent?: number, successPercent?: number) => React.ReactNode;
//   status?: typeof ProgressStatuses[number];
//   showInfo?: boolean;
//   strokeWidth?: number;
//   strokeLinecap?: 'butt' | 'square' | 'round';
//   strokeColor?: string | string[] | ProgressGradient;
//   trailColor?: string;
//   width?: number;
//   success?: SuccessProps;
//   style?: React.CSSProperties;
//   gapDegree?: number;
//   gapPosition?: 'top' | 'bottom' | 'left' | 'right';
//   size?: ProgressSize;
//   steps?: number;
//   /** @deprecated Use `success` instead */
//   successPercent?: number;
//   children?: React.ReactNode;
// }

// const Progress: React.FC<ProgressProps> = (props: ProgressProps) => {
//   const {
//     prefixCls: customizePrefixCls,
//     className,
//     steps,
//     strokeColor,
//     percent = 0,
//     size = 'default',
//     showInfo = true,
//     type = 'line',
//     ...restProps
//   } = props;

//   function getPercentNumber() {
//     const successPercent = getSuccessPercent(props);
//     return parseInt(
//       successPercent !== undefined ? successPercent.toString() : percent.toString(),
//       10,
//     );
//   }

//   function getProgressStatus() {
//     const { status } = props;
//     if (ProgressStatuses.indexOf(status!) < 0 && getPercentNumber() >= 100) {
//       return 'success';
//     }
//     return status || 'normal';
//   }

//   function renderProcessInfo(prefixCls: string, progressStatus: typeof ProgressStatuses[number]) {
//     const { format } = props;
//     const successPercent = getSuccessPercent(props);
//     if (!showInfo) {
//       return null;
//     }
//     let text;
//     const textFormatter = format || (percentNumber => `${percentNumber}%`);
//     const isLineType = type === 'line';
//     if (format || (progressStatus !== 'exception' && progressStatus !== 'success')) {
//       text = textFormatter(validProgress(percent), validProgress(successPercent));
//     } else if (progressStatus === 'exception') {
//       text = isLineType ? <CloseCircleFilled /> : <CloseOutlined />;
//     } else if (progressStatus === 'success') {
//       text = isLineType ? <CheckCircleFilled /> : <CheckOutlined />;
//     }
//     return (
//       <span className={`${prefixCls}-text`} title={typeof text === 'string' ? text : undefined}>
//         {text}
//       </span>
//     );
//   }

//   const { getPrefixCls, direction } = React.useContext(ConfigContext);

//   const prefixCls = getPrefixCls('progress', customizePrefixCls);
//   const progressStatus = getProgressStatus();
//   const progressInfo = renderProcessInfo(prefixCls, progressStatus);

//   warning(
//     !('successPercent' in props),
//     'Progress',
//     '`successPercent` is deprecated. Please use `success.percent` instead.',
//   );

//   const strokeColorNotArray = Array.isArray(strokeColor) ? strokeColor[0] : strokeColor;
//   const strokeColorNotGradient =
//     typeof strokeColor === 'string' || Array.isArray(strokeColor) ? strokeColor : undefined;
//   let progress;
//   // Render progress shape
//   if (type === 'line') {
//     progress = steps ? (
//       <Steps {...props} strokeColor={strokeColorNotGradient} prefixCls={prefixCls} steps={steps}>
//         {progressInfo}
//       </Steps>
//     ) : (
//       <Line
//         {...props}
//         strokeColor={strokeColorNotArray}
//         prefixCls={prefixCls}
//         direction={direction}
//       >
//         {progressInfo}
//       </Line>
//     );
//   } else if (type === 'circle' || type === 'dashboard') {
//     progress = (
//       <Circle
//         {...props}
//         strokeColor={strokeColorNotArray}
//         prefixCls={prefixCls}
//         progressStatus={progressStatus}
//       >
//         {progressInfo}
//       </Circle>
//     );
//   }

//   const classString = classNames(
//     prefixCls,
//     {
//       [`${prefixCls}-${(type === 'dashboard' && 'circle') || (steps && 'steps') || type}`]: true,
//       [`${prefixCls}-status-${progressStatus}`]: true,
//       [`${prefixCls}-show-info`]: showInfo,
//       [`${prefixCls}-${size}`]: size,
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//     },
//     className,
//   );

//   return (
//     <div
//       {...omit(restProps, [
//         'status',
//         'format',
//         'trailColor',
//         'strokeWidth',
//         'width',
//         'gapDegree',
//         'gapPosition',
//         'strokeLinecap',
//         'success',
//         'successPercent',
//       ])}
//       className={classString}
//     >
//       {progress}
//     </div>
//   );
// };

// export default Progress;

// import warning from '../_util/warning';

// export function validProgress(progress: number | undefined) {
//   if (!progress || progress < 0) {
//     return 0;
//   }
//   if (progress > 100) {
//     return 100;
//   }
//   return progress;
// }

// export function getSuccessPercent({
//   success,
//   successPercent,
// }: {
//   success?: {
//     progress?: number;
//     percent?: number;
//   };
//   successPercent?: number;
// }) {
//   let percent = successPercent;
//   /** @deprecated Use `percent` instead */
//   if (success && 'progress' in success) {
//     warning(
//       false,
//       'Progress',
//       '`success.progress` is deprecated. Please use `success.percent` instead.',
//     );
//     percent = success.progress;
//   }
//   if (success && 'percent' in success) {
//     percent = success.percent;
//   }
//   return percent;
// }
