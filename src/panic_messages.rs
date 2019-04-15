pub(crate) const INVALID_ENUM: &str =
    "[discord_game_sdk] could not match enum value to any definition, this crate might be out of \
     date as the SDK is sending incompatible data";

pub(crate) const NO_NUL: &str =
    "[discord_game_sdk] received a string from the SDK that was not NUL terminated";

pub(crate) const NOT_UTF8: &str =
    "[discord_game_sdk] received a string from the SDK that was not valid UTF-8";

pub(crate) const NULL_PTR: &str =
    "[discord_game_sdk] received a NULL pointer where a valid pointer is expected";

pub(crate) const SEND_FAIL: &str =
    "[discord_game_sdk] failed to send a message across FFI bounds, most likely because the \
     Discord value was dropped";
