use crate::sys;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cast {
    Number,
    String,
}

impl Into<sys::EDiscordLobbySearchCast> for Cast {
    fn into(self) -> sys::EDiscordLobbySearchCast {
        match self {
            Cast::String => sys::DiscordLobbySearchCast_String,
            Cast::Number => sys::DiscordLobbySearchCast_Number,
        }
    }
}
