use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use pg_crud_cmn::{DfltSomeOneEl, take_fst_dup};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Error, Location)] //todo , Optml
pub enum UnqVecTryNewEr<T> {
    NotUnq {
        #[eo_to_err_string_serde]
        v: T,
        loc: Loc,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema, JsonSchema, Optml)]
pub struct UnqVec<T>(Vec<T>);
impl<T> UnqVec<T> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
}
impl<T> AsRef<[T]> for UnqVec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T: PartialEq> UnqVec<T> {
    pub fn try_new(mut v: Vec<T>) -> Result<Self, UnqVecTryNewEr<T>> {
        if let Some(duplicate) = take_fst_dup(&mut v) {
            return Err(UnqVecTryNewEr::NotUnq {
                v: duplicate,
                loc: loc!(),
            });
        }
        Ok(Self(v))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
        _serde::Deserialize<'de> for UnqVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(clippy::single_call_fn)] // shared converter keeps try_new->serde::de::Error mapping in one place for all visitor entry points
            fn try_new_or_de_error<T, E>(v: Vec<T>) -> Result<UnqVec<T>, E>
            where
                T: std::fmt::Debug + PartialEq + Clone,
                E: _serde::de::Error,
            {
                UnqVec::try_new(v).map_err(|er| E::custom(format!("{er:?}")))
            }
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<UnqVec<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = UnqVec<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "tuple struct UnqVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    try_new_or_de_error(f0)
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct UnqVec with 1 el",
                        ));
                    };
                    try_new_or_de_error(f0)
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "UnqVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: DfltSomeOneEl> DfltSomeOneEl for UnqVec<T> {
    fn dflt_some_one_el() -> Self {
        Self(vec![DfltSomeOneEl::dflt_some_one_el()])
    }
}
impl<T> Default for UnqVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<UnqVec<T>> for Vec<T> {
    fn from(v: UnqVec<T>) -> Self {
        v.0
    }
}
impl<T1> UnqVec<T1> {
    pub fn from_t1_impl_from_t2<T2: From<T1>>(v: Self) -> UnqVec<T2> {
        UnqVec(v.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
#[cfg(test)]
mod tests {
    use super::{UnqVec, UnqVecTryNewEr};
    use pg_crud_cmn::{first_duplicate_idx, take_fst_dup};
    #[derive(Debug, PartialEq, Eq)]
    struct NonClone(u8);
    #[test]
    fn try_new_returns_ok_for_unq_values() {
        let v = UnqVec::try_new(vec![1i32, 2i32, 3i32]).expect("90a6f3e1");
        assert_eq!(v.to_vec(), &vec![1i32, 2i32, 3i32]);
    }
    #[test]
    fn try_new_returns_not_unq_for_duplicate() {
        let er = UnqVec::try_new(vec![1i32, 2i32, 1i32]).expect_err("9230d2a3");
        match er {
            UnqVecTryNewEr::NotUnq { v, .. } => assert_eq!(v, 1i32),
        }
    }
    #[test]
    fn try_new_returns_first_repeated_value_when_many_duplicates_exist() {
        let er = UnqVec::try_new(vec![3i32, 4i32, 4i32, 3i32]).expect_err("d22e7b7a");
        match er {
            UnqVecTryNewEr::NotUnq { v, .. } => assert_eq!(v, 4i32),
        }
    }
    #[test]
    fn try_new_supports_non_clone_values() {
        let er =
            UnqVec::try_new(vec![NonClone(1), NonClone(2), NonClone(1)]).expect_err("0ed6be5b");
        match er {
            UnqVecTryNewEr::NotUnq { v, .. } => assert_eq!(v, NonClone(1)),
        }
    }
    #[test]
    fn default_and_is_empty_are_consistent() {
        let v = UnqVec::<i32>::default();
        assert!(v.is_empty());
        assert!(v.to_vec().is_empty());
        assert!(v.as_slice().is_empty());
    }
    #[test]
    fn into_vec_preserves_inner_values() {
        let v = UnqVec::try_new(vec![4i32, 5i32]).expect("736fd1f4");
        let actual = v.into_vec();
        assert_eq!(actual, vec![4i32, 5i32]);
    }
    #[test]
    fn from_t1_impl_from_t2_converts_elements() {
        let src = UnqVec::try_new(vec![1i32, 2i32]).expect("ab4976d9");
        let actual = UnqVec::from_t1_impl_from_t2::<i64>(src);
        assert_eq!(actual.into_vec(), vec![1i64, 2i64]);
    }
    #[test]
    fn first_duplicate_idx_returns_none_for_unq_input() {
        let v = vec![1u8, 2u8, 3u8];
        assert!(first_duplicate_idx(&v).is_none());
    }
    #[test]
    fn first_duplicate_idx_returns_first_repeated_value_index() {
        let v = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(first_duplicate_idx(&v), Some(2usize));
    }
    #[test]
    fn take_fst_dup_returns_none_for_unq_input() {
        let mut values = vec![1u8, 2u8, 3u8];
        let actual = take_fst_dup(&mut values);
        assert!(actual.is_none());
        assert_eq!(values, vec![1u8, 2u8, 3u8]);
    }
    #[test]
    fn take_fst_dup_returns_first_duplicate_value() {
        let mut values = vec![7u8, 8u8, 8u8, 7u8];
        let actual = take_fst_dup(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(values.len(), 3usize);
    }
    #[test]
    fn as_slice_matches_to_vec_view() {
        let v = UnqVec::try_new(vec![1u8, 2u8, 3u8]).expect("0ec4e973");
        assert_eq!(v.as_slice(), &[1u8, 2u8, 3u8]);
        assert_eq!(v.as_slice(), v.to_vec().as_slice());
    }
    #[test]
    fn as_slice_iter_traverses_values_in_order() {
        let v = UnqVec::try_new(vec![4u8, 5u8, 6u8]).expect("8b0ae3f4");
        let actual = v.as_slice().to_vec();
        assert_eq!(actual, vec![4u8, 5u8, 6u8]);
    }
    #[test]
    fn as_ref_slice_matches_as_slice() {
        let v = UnqVec::try_new(vec![9u8, 8u8, 7u8]).expect("ad547bd8");
        assert_eq!(v.as_ref(), v.as_slice());
    }
}
