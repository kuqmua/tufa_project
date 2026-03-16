use optml::Optml;
use serde::{Deserialize, Deserializer, Serialize, de::Error as SerdeEr};
pub use server_port_try_from_u16::server_port_try_from_u16;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
pub const SERVER_PORT_MIN_VALUE: u16 = 1024;
pub const SERVER_PORT_MAX_VALUE: u16 = 49151;
const SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE: &str =
    "init failed, reason: system port range 0-1023";
const SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE: &str =
    "init failed, reason: ephemeral port range 49152-65535";
#[derive(Debug, Clone, Copy, Serialize, Optml)]
pub struct ServerPort(u16);
impl Display for ServerPort {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize, Optml)]
pub struct ServerPortEr {
    msg: String,
    server_port_max: u16,
    server_port_min: u16,
    v: u16,
}
impl Display for ServerPortEr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "server_port_min: {}, server_port_max: {}, v: {}",
            self.server_port_min, self.server_port_max, self.v
        )
    }
}
impl Error for ServerPortEr {}
impl TryFrom<u16> for ServerPort {
    type Error = ServerPortEr;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v < SERVER_PORT_MIN_VALUE {
            Err(Self::Error {
                server_port_min: SERVER_PORT_MIN_VALUE,
                server_port_max: SERVER_PORT_MAX_VALUE,
                v,
                msg: String::from(SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE),
            })
        } else if v <= SERVER_PORT_MAX_VALUE {
            Ok(Self(v))
        } else {
            Err(Self::Error {
                server_port_min: SERVER_PORT_MIN_VALUE,
                server_port_max: SERVER_PORT_MAX_VALUE,
                v,
                msg: String::from(SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE),
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
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        }) {
            Ok(v) => Ok(v),
            Err(er) => Err(SerdeEr::custom(er)),
        }
    }
}
