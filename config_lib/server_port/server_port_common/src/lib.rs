pub const SERVER_PORT_MIN_VALUE: std::primitive::u16 = 1024; //system port range 0-1023
pub const SERVER_PORT_MAX_VALUE: std::primitive::u16 = 49151; //ephemeral port range 49152-65535
pub const SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE: &str =
    "initialization failed, reason: system port range 0-1023";
pub const SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE: &str =
    "initialization failed, reason: ephemeral port range 49152-65535";
