// #[derive(thiserror::Error)]
// pub enum OneError {
//     One(String)
// }
// impl std::fmt::Display for OneError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn one() -> Result<(), OneError> {
//     Err(OneError::One(String::from("one")))
// }
// #[derive(thiserror::Error)]
// pub enum TwoError {
//     Two(String)
// }
// impl std::fmt::Display for TwoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn two() -> Result<(), TwoError> {
//     Err(TwoError::Two(String::from("two")))
// }
// #[derive(thiserror::Error)]
// pub enum ThreeError {
//     One {
//         #[source]
//         one: OneError,
//     },
//     Two {
//         #[source]
//         two: TwoError,
//     },
//     OneTwo {
//         #[source]
//         one: OneError,
//         // #[source] //error: duplicate #[source] attribute - what to do with that?
//         two: TwoError,
//     }
// }
// impl std::fmt::Display for ThreeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }
// pub async fn three() -> Result<(), ThreeError> {
//     match futures::join!(
//         one(),
//         two()
//     ) {
//         (Ok(_), Ok(_)) => Ok(()),
//         (Err(one), Ok(_)) => Err(ThreeError::One {
//             one,
//         }),
//         (Ok(_), Err(two)) => Err(ThreeError::Two {
//             two,
//         }),
//         (Err(one), Err(two)) => Err(ThreeError::OneTwo {
//             //what to do with that?
//             one,
//             two,
//         }),
//     }
// }

pub async fn dev() {}
