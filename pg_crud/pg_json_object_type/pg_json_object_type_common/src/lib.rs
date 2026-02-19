use location_lib::{Location, code_occurence, code_occurence::CodeOccurence};
use pg_crud_common::DefaultOptionSomeVecOneEl;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Error, Location)]
pub enum UniqueVecTryNewEr<T> {
    NotUnique {
        #[eo_to_err_string_serde]
        value: T,
        code_occurence: CodeOccurence,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema, JsonSchema)]
pub struct UniqueVec<T>(Vec<T>);
impl<T: PartialEq + Clone> UniqueVec<T> {
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
    pub fn try_new(value: Vec<T>) -> Result<Self, UniqueVecTryNewEr<T>> {
        let mut acc_4855bea7 = Vec::new();
        for el_4dddc7c0 in &value {
            if acc_4855bea7.contains(&el_4dddc7c0) {
                return Err(UniqueVecTryNewEr::NotUnique {
                    value: el_4dddc7c0.clone(),
                    code_occurence: code_occurence!(),
                });
            }
            acc_4855bea7.push(el_4dddc7c0);
        }
        Ok(Self(value))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
        _serde::Deserialize<'de> for UniqueVec<T>
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
                marker: _serde::__private228::PhantomData<UniqueVec<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = UniqueVec<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "tuple struct UniqueVec",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    match UniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct UniqueVec with 1 element",
                        ));
                    };
                    match UniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "UniqueVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: DefaultOptionSomeVecOneEl> DefaultOptionSomeVecOneEl for UniqueVec<T> {
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl<T> Default for UniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<UniqueVec<T>> for Vec<T> {
    fn from(value: UniqueVec<T>) -> Self {
        value.0
    }
}
impl<T1> UniqueVec<T1> {
    pub fn from_t1_impl_from_t2<T2: From<T1>>(value: Self) -> UniqueVec<T2> {
        UniqueVec(value.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
