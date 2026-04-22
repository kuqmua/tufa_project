use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use pg_crud_cmn::DfltSomeOneEl;
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
impl<T: PartialEq + Clone> UnqVec<T> {
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
    pub fn try_new(v: Vec<T>) -> Result<Self, UnqVecTryNewEr<T>> {
        let mut acc: Vec<&T> = Vec::with_capacity(v.len());
        for el in &v {
            if acc.contains(&el) {
                return Err(UnqVecTryNewEr::NotUnq {
                    v: el.clone(),
                    loc: loc!(),
                });
            }
            acc.push(el);
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
                    match UnqVec::try_new(f0) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
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
                    match UnqVec::try_new(f0) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
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
    fn default_and_is_empty_are_consistent() {
        let v = UnqVec::<i32>::default();
        assert!(v.is_empty());
        assert!(v.to_vec().is_empty());
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
}
