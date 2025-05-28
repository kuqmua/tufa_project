generate_postgresql_json_types::generate_postgresql_json_types!([
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "NotNull"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension1": {
                "dimension1_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": {
            "ArrayDimension4": {
                "dimension1_not_null_or_nullable": "Nullable",
                "dimension2_not_null_or_nullable": "Nullable",
                "dimension3_not_null_or_nullable": "Nullable",
                "dimension4_not_null_or_nullable": "Nullable"
            }
        }
    },
    {
        "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "UuidUuidAsJsonbString",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
]);
//////////
//////////
//////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI8AsNotNullJsonbNumberRead(StdPrimitiveI8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI8AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::i8 {
        self.0.0
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberRead(OptionStdPrimitiveI8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI8AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::i8> {
        match self.0.0 {
            Some(value) => Some(value.0),
            None => None
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::i8> {
        self.0.0.into_iter().map(|element|{element.0}).collect()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::i8>> {
        self.0.0.into_iter().map(|element|match element.0{
            Some(value) => Some(value.0),
            None => None
        }).collect()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::i8>> {
        match self.0.0 {
            Some(value) => Some(value.0.into_iter().map(|element|{element.0}).collect()),
            None => None
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>> {
        match self.0.0 {
            Some(value) => Some(value.0.into_iter().map(|element|match element.0{
                Some(value) => Some(value.0),
                None => None
            }).collect()),
            None => None
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::i8>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i8>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI16AsNotNullJsonbNumberRead(StdPrimitiveI16AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI16AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::i16 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI16AsNullableJsonbNumberRead(OptionStdPrimitiveI16AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI16AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::i16> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::i16> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::i16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::i16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::i16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i16>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI32AsNotNullJsonbNumberRead(StdPrimitiveI32AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI32AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::i32 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI32AsNullableJsonbNumberRead(OptionStdPrimitiveI32AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI32AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::i32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::i32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::i32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::i32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::i32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i32>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveI64AsNotNullJsonbNumberRead(StdPrimitiveI64AsNotNullJsonbNumberOrigin);
impl StdPrimitiveI64AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::i64 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI64AsNullableJsonbNumberRead(OptionStdPrimitiveI64AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveI64AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::i64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::i64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::i64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::i64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::i64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::i64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::i64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::i64>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveU8AsNotNullJsonbNumberRead(StdPrimitiveU8AsNotNullJsonbNumberOrigin);
impl StdPrimitiveU8AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::u8 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveU8AsNullableJsonbNumberRead(OptionStdPrimitiveU8AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveU8AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::u8> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u8> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::u8>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::u8>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::u8>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u8>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u8>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u8>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveU16AsNotNullJsonbNumberRead(StdPrimitiveU16AsNotNullJsonbNumberOrigin);
impl StdPrimitiveU16AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::u16 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveU16AsNullableJsonbNumberRead(OptionStdPrimitiveU16AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveU16AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::u16> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u16> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::u16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::u16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::u16>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u16>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u16>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u16>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveU32AsNotNullJsonbNumberRead(StdPrimitiveU32AsNotNullJsonbNumberOrigin);
impl StdPrimitiveU32AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::u32 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveU32AsNullableJsonbNumberRead(OptionStdPrimitiveU32AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveU32AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::u32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::u32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::u32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::u32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u32>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveU64AsNotNullJsonbNumberRead(StdPrimitiveU64AsNotNullJsonbNumberOrigin);
impl StdPrimitiveU64AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::u64 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveU64AsNullableJsonbNumberRead(OptionStdPrimitiveU64AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveU64AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::u64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::u64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::u64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::u64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::u64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::u64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::u64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::u64>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveF32AsNotNullJsonbNumberRead(StdPrimitiveF32AsNotNullJsonbNumberOrigin);
impl StdPrimitiveF32AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::f32 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveF32AsNullableJsonbNumberRead(OptionStdPrimitiveF32AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveF32AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::f32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::f32> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::f32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::f32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::f32>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f32>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f32>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f32>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveF64AsNotNullJsonbNumberRead(StdPrimitiveF64AsNotNullJsonbNumberOrigin);
impl StdPrimitiveF64AsNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::primitive::f64 {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveF64AsNullableJsonbNumberRead(OptionStdPrimitiveF64AsNullableJsonbNumberOrigin);
impl OptionStdPrimitiveF64AsNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::f64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumberRead(VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::f64> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumberRead(VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::f64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumberRead(OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::f64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::f64>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::f64>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::f64>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumberRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::f64>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdPrimitiveBoolAsNotNullJsonbBooleanRead(StdPrimitiveBoolAsNotNullJsonbBooleanOrigin);
impl StdPrimitiveBoolAsNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::primitive::bool {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveBoolAsNullableJsonbBooleanRead(OptionStdPrimitiveBoolAsNullableJsonbBooleanOrigin);
impl OptionStdPrimitiveBoolAsNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::primitive::bool> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBooleanRead(VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::primitive::bool> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBooleanRead(VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::primitive::bool>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::primitive::bool>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBooleanRead(OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::primitive::bool>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::primitive::bool>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::primitive::bool>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBooleanRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::primitive::bool>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdStringStringAsNotNullJsonbStringRead(StdStringStringAsNotNullJsonbStringOrigin);
impl StdStringStringAsNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::string::String {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdStringStringAsNullableJsonbStringRead(OptionStdStringStringAsNullableJsonbStringOrigin);
impl OptionStdStringStringAsNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::string::String> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfStdStringStringAsNotNullArrayOfNotNullJsonbStringRead(VecOfStdStringStringAsNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfStdStringStringAsNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::string::String> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbStringRead(VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::string::String>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbStringRead(OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::string::String>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::string::String>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(
    VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin,
);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(
    OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin,
);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::string::String>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(
    OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin,
);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::string::String>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(
    OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::string::String>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::string::String>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::string::String>>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct UuidUuidAsNotNullJsonbStringRead(UuidUuidAsNotNullJsonbStringOrigin);
impl UuidUuidAsNotNullJsonbStringRead {
    pub fn into_inner(self) -> uuid::Uuid {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionUuidUuidAsNullableJsonbStringRead(OptionUuidUuidAsNullableJsonbStringOrigin);
impl OptionUuidUuidAsNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<uuid::Uuid> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfUuidUuidAsNotNullArrayOfNotNullJsonbStringRead(VecOfUuidUuidAsNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfUuidUuidAsNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<uuid::Uuid> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbStringRead(VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<uuid::Uuid>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbStringRead(OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<uuid::Uuid>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<uuid::Uuid>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<uuid::Uuid>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead(OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringOrigin);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<uuid::Uuid>>>>>>>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead(
    OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringOrigin,
);
impl OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbStringRead {
    pub fn into_inner(self) -> std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<std::vec::Vec<std::option::Option<uuid::Uuid>>>>>>>>> {
        todo!()
    }
}
