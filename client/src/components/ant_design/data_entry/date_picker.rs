// // import * as React from 'react';
// // import type { ButtonProps } from '../button';
// // import Button from '../button';

// // export default function PickerButton(props: ButtonProps) {
// //   return <Button size="small" type="primary" {...props} />;
// // }

// // import * as React from 'react';
// // import type { TagProps } from '../tag';
// // import Tag from '../tag';

// // export default function PickerTag(props: TagProps) {
// //   return <Tag color="blue" {...props} />;
// // }

// // import type { Moment } from 'moment';
// // import momentGenerateConfig from 'rc-picker/lib/generate/moment';
// // import type {
// //   PickerDateProps,
// //   PickerProps,
// //   RangePickerProps as BaseRangePickerProps,
// // } from './generatePicker';
// // import generatePicker from './generatePicker';

// // export type DatePickerProps = PickerProps<Moment>;
// // export type MonthPickerProps = Omit<PickerDateProps<Moment>, 'picker'>;
// // export type WeekPickerProps = Omit<PickerDateProps<Moment>, 'picker'>;
// // export type RangePickerProps = BaseRangePickerProps<Moment>;

// // const DatePicker = generatePicker<Moment>(momentGenerateConfig);

// // export default DatePicker;

// import type { PickerMode } from 'rc-picker/lib/interface';
// import type { DirectionType } from '../config-provider';
// import type { SelectCommonPlacement } from '../_util/motion';
// import type { PickerLocale } from './generatePicker';

// export function getPlaceholder(
//   picker: PickerMode | undefined,
//   locale: PickerLocale,
//   customizePlaceholder?: string,
// ): string {
//   if (customizePlaceholder !== undefined) {
//     return customizePlaceholder;
//   }

//   if (picker === 'year' && locale.lang.yearPlaceholder) {
//     return locale.lang.yearPlaceholder;
//   }
//   if (picker === 'quarter' && locale.lang.quarterPlaceholder) {
//     return locale.lang.quarterPlaceholder;
//   }
//   if (picker === 'month' && locale.lang.monthPlaceholder) {
//     return locale.lang.monthPlaceholder;
//   }
//   if (picker === 'week' && locale.lang.weekPlaceholder) {
//     return locale.lang.weekPlaceholder;
//   }
//   if (picker === 'time' && locale.timePickerLocale.placeholder) {
//     return locale!.timePickerLocale.placeholder;
//   }
//   return locale.lang.placeholder;
// }

// export function getRangePlaceholder(
//   picker: PickerMode | undefined,
//   locale: PickerLocale,
//   customizePlaceholder?: [string, string],
// ) {
//   if (customizePlaceholder !== undefined) {
//     return customizePlaceholder;
//   }

//   if (picker === 'year' && locale.lang.yearPlaceholder) {
//     return locale.lang.rangeYearPlaceholder;
//   }
//   if (picker === 'quarter' && locale.lang.quarterPlaceholder) {
//     return locale.lang.rangeQuarterPlaceholder;
//   }
//   if (picker === 'month' && locale.lang.monthPlaceholder) {
//     return locale.lang.rangeMonthPlaceholder;
//   }
//   if (picker === 'week' && locale.lang.weekPlaceholder) {
//     return locale.lang.rangeWeekPlaceholder;
//   }
//   if (picker === 'time' && locale.timePickerLocale.placeholder) {
//     return locale!.timePickerLocale.rangePlaceholder;
//   }
//   return locale.lang.rangePlaceholder;
// }

// export function transPlacement2DropdownAlign(
//   direction: DirectionType,
//   placement?: SelectCommonPlacement,
// ) {
//   const overflow = {
//     adjustX: 1,
//     adjustY: 1,
//   };
//   switch (placement) {
//     case 'bottomLeft': {
//       return {
//         points: ['tl', 'bl'],
//         offset: [0, 4],
//         overflow,
//       };
//     }
//     case 'bottomRight': {
//       return {
//         points: ['tr', 'br'],
//         offset: [0, 4],
//         overflow,
//       };
//     }
//     case 'topLeft': {
//       return {
//         points: ['bl', 'tl'],
//         offset: [0, -4],
//         overflow,
//       };
//     }
//     case 'topRight': {
//       return {
//         points: ['br', 'tr'],
//         offset: [0, -4],
//         overflow,
//       };
//     }
//     default: {
//       return direction === 'rtl'
//         ? {
//             points: ['tr', 'br'],
//             offset: [0, 4],
//             overflow,
//           }
//         : {
//             points: ['tl', 'bl'],
//             offset: [0, 4],
//             overflow,
//           };
//     }
//   }
// }
