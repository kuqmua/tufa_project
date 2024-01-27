// import LoadingOutlined from '@ant-design/icons/LoadingOutlined';
// import classNames from 'classnames';
// import * as React from 'react';

// import { ConfigContext } from '../config-provider';
// import { cloneElement } from '../_util/reactNode';
// import type { TimelineItemProps } from './TimelineItem';
// import TimelineItem from './TimelineItem';

// export interface TimelineProps {
//   prefixCls?: string;
//   className?: string;
//   /** 指定最后一个幽灵节点是否存在或内容 */
//   pending?: React.ReactNode;
//   pendingDot?: React.ReactNode;
//   style?: React.CSSProperties;
//   reverse?: boolean;
//   mode?: 'left' | 'alternate' | 'right';
//   children?: React.ReactNode;
// }

// interface TimelineType extends React.FC<TimelineProps> {
//   Item: React.FC<TimelineItemProps>;
// }

// const Timeline: TimelineType = props => {
//   const { getPrefixCls, direction } = React.useContext(ConfigContext);
//   const {
//     prefixCls: customizePrefixCls,
//     pending = null,
//     pendingDot,
//     children,
//     className,
//     reverse = false,
//     mode = '' as TimelineProps['mode'],
//     ...restProps
//   } = props;
//   const prefixCls = getPrefixCls('timeline', customizePrefixCls);
//   const pendingNode = typeof pending === 'boolean' ? null : pending;

//   const pendingItem = pending ? (
//     <TimelineItem pending={!!pending} dot={pendingDot || <LoadingOutlined />}>
//       {pendingNode}
//     </TimelineItem>
//   ) : null;

//   const timeLineItems = React.Children.toArray(children);
//   timeLineItems.push(pendingItem as any);
//   if (reverse) {
//     timeLineItems.reverse();
//   }

//   const getPositionCls = (ele: React.ReactElement<any>, idx: number) => {
//     if (mode === 'alternate') {
//       if (ele.props.position === 'right') return `${prefixCls}-item-right`;
//       if (ele.props.position === 'left') return `${prefixCls}-item-left`;
//       return idx % 2 === 0 ? `${prefixCls}-item-left` : `${prefixCls}-item-right`;
//     }
//     if (mode === 'left') return `${prefixCls}-item-left`;
//     if (mode === 'right') return `${prefixCls}-item-right`;
//     if (ele.props.position === 'right') return `${prefixCls}-item-right`;
//     return '';
//   };

//   // Remove falsy items
//   const truthyItems = timeLineItems.filter(item => !!item);
//   const itemsCount = React.Children.count(truthyItems);
//   const lastCls = `${prefixCls}-item-last`;
//   const items = React.Children.map(truthyItems, (ele: React.ReactElement<any>, idx) => {
//     const pendingClass = idx === itemsCount - 2 ? lastCls : '';
//     const readyClass = idx === itemsCount - 1 ? lastCls : '';
//     return cloneElement(ele, {
//       className: classNames([
//         ele.props.className,
//         !reverse && !!pending ? pendingClass : readyClass,
//         getPositionCls(ele, idx),
//       ]),
//     });
//   });

//   const hasLabelItem = timeLineItems.some((item: React.ReactElement<any>) => !!item?.props?.label);

//   const classString = classNames(
//     prefixCls,
//     {
//       [`${prefixCls}-pending`]: !!pending,
//       [`${prefixCls}-reverse`]: !!reverse,
//       [`${prefixCls}-${mode}`]: !!mode && !hasLabelItem,
//       [`${prefixCls}-label`]: hasLabelItem,
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//     },
//     className,
//   );

//   return (
//     <ul {...restProps} className={classString}>
//       {items}
//     </ul>
//   );
// };

// Timeline.Item = TimelineItem;

// export default Timeline;

// import classNames from 'classnames';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';

// export interface TimelineItemProps {
//   prefixCls?: string;
//   className?: string;
//   color?: string;
//   dot?: React.ReactNode;
//   pending?: boolean;
//   position?: string;
//   style?: React.CSSProperties;
//   label?: React.ReactNode;
//   children?: React.ReactNode;
// }

// // for compatibility
// // https://github.com/ant-design/ant-design/pull/26832
// export interface TimeLineItemProps extends TimelineItemProps {
//   __deprecated_do_not_use_it__?: any; // eslint-disable-line camelcase
// }

// const TimelineItem: React.FC<TimelineItemProps> = ({
//   prefixCls: customizePrefixCls,
//   className,
//   color = 'blue',
//   dot,
//   pending = false,
//   position /** Dead, but do not pass in <li {...omit()} */,
//   label,
//   children,
//   ...restProps
// }) => {
//   const { getPrefixCls } = React.useContext(ConfigContext);

//   const prefixCls = getPrefixCls('timeline', customizePrefixCls);
//   const itemClassName = classNames(
//     {
//       [`${prefixCls}-item`]: true,
//       [`${prefixCls}-item-pending`]: pending,
//     },
//     className,
//   );

//   const dotClassName = classNames({
//     [`${prefixCls}-item-head`]: true,
//     [`${prefixCls}-item-head-custom`]: !!dot,
//     [`${prefixCls}-item-head-${color}`]: true,
//   });

//   const customColor = /blue|red|green|gray/.test(color || '') ? undefined : color;

//   return (
//     <li {...restProps} className={itemClassName}>
//       {label && <div className={`${prefixCls}-item-label`}>{label}</div>}
//       <div className={`${prefixCls}-item-tail`} />
//       <div className={dotClassName} style={{ borderColor: customColor, color: customColor }}>
//         {dot}
//       </div>
//       <div className={`${prefixCls}-item-content`}>{children}</div>
//     </li>
//   );
// };

// export default TimelineItem;

// import Timeline from './Timeline';

// export { TimelineProps } from './Timeline';
// export { TimelineItemProps } from './TimelineItem';

// export default Timeline;
