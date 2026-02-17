use serde::{Deserialize, Deserializer, Serialize, de::Error as SerdeError};
use server_port_common::{
    SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE,
    SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE, SERVER_PORT_MAX_VALUE, SERVER_PORT_MIN_VALUE,
};
pub use server_port_try_from_u16::server_port_try_from_u16;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ServerPort(u16);
impl Display for ServerPort {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPortError {
    message: String,
    server_port_max_value: u16,
    server_port_min_value: u16,
    value: u16,
}
impl Display for ServerPortError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "server_port_min_value: {}, server_port_max_value: {}, value: {}",
            self.server_port_min_value, self.server_port_max_value, self.value
        )
    }
}
impl Error for ServerPortError {}
impl TryFrom<u16> for ServerPort {
    type Error = ServerPortError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < SERVER_PORT_MIN_VALUE {
            Err(Self::Error {
                server_port_min_value: SERVER_PORT_MIN_VALUE,
                server_port_max_value: SERVER_PORT_MAX_VALUE,
                value,
                message: String::from(SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE),
            })
        } else if value <= SERVER_PORT_MAX_VALUE {
            Ok(Self(value))
        } else {
            Err(Self::Error {
                server_port_min_value: SERVER_PORT_MIN_VALUE,
                server_port_max_value: SERVER_PORT_MAX_VALUE,
                value,
                message: String::from(SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE),
            })
        }
    }
}
#[allow(clippy::absolute_paths)]
impl<'de> Deserialize<'de> for ServerPort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Self::try_from(match u16::deserialize(deserializer) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        }) {
            Ok(value) => Ok(value),
            Err(error) => Err(SerdeError::custom(error)),
        }
    }
}
