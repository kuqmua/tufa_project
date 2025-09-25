pub use generate_postgresql_json_object_type::GeneratePostgresqlJsonObjectType;
pub use generate_postgresql_json_object_type::postgresql_json_object_type_pattern;

#[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum UniqueVecTryNewErrorNamed<T> {
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: T,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct UniqueVec<T>(std::vec::Vec<T>);
impl<T: std::cmp::PartialEq + Clone> UniqueVec<T> {
    pub fn try_new(value: std::vec::Vec<T>) -> Result<Self, UniqueVecTryNewErrorNamed<T>> {
        let mut acc = vec![];
        for element in &value {
            if acc.contains(&element) {
                return Err(UniqueVecTryNewErrorNamed::NotUnique {
                    value: element.clone(),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            acc.push(element);
        }
        Ok(Self(value))
    }
    pub const fn to_vec(&self) -> &std::vec::Vec<T> {
        &self.0
    }
    pub fn into_vec(self) -> std::vec::Vec<T> {
        self.0
    }
    pub const fn is_empty(&self) -> std::primitive::bool {
        self.0.is_empty()
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for UniqueVec<T>
    where
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<UniqueVec<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = UniqueVec<T>;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct UniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::vec::Vec<T> = <std::vec::Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    match UniqueVec::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<T>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct UniqueVec with 1 element"));
                        }
                    };
                    match UniqueVec::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "UniqueVec",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UniqueVec<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl<T> std::default::Default for UniqueVec<T> {
    fn default() -> Self {
        Self(std::vec::Vec::default())
    }
}
impl<T> std::convert::From<UniqueVec<T>> for Vec<T> {
    fn from(value: UniqueVec<T>) -> Self {
        value.0
    }
}

impl<T1> UniqueVec<T1> {
    pub fn from_t1_impl_from_t2<T2: std::convert::From<T1>>(value: UniqueVec<T1>) -> UniqueVec<T2> {
        UniqueVec(
            value.0
            .into_iter()
            .map(T2::from)
            .collect::<std::vec::Vec<T2>>()
        )
    }
}
