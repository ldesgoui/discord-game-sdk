//! Safe wrapper for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide).
//!
//! # Status
//!
//! This library is currently in very early stages, most of the API is missing.
//!
//! # "Legal" note
//!
//! This wrapper was informally allowed for publication and distribution by Discord Staff.
//! I cannot redistribute the SDK files until it is made open-source or is licensed for redistribution. You will have to follow some instructions when first setting up your project.
//! This also means that docs.rs will not be able to build the documentation.
//! Apologies for the inconvenience.
//!
//! If you're a part of Discord and wish to discuss this, please email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.

#![allow(unused_variables, unused_imports)]

// This absolutely needs to come first
#[macro_use]
mod macros;

mod action;
mod activity_change;
mod create_flags;
mod discord;
pub mod error;
pub mod event;
mod oauth2_token;
mod premium_type;
mod request_reply;
mod user;
mod utils;
mod methods {
    mod core;

    mod activities;
    mod applications;
    mod users;
}

mod prelude {
    pub(crate) use crate::discord::Discord;
    pub(crate) use crate::error::{
        BindingsViolation, DeveloperViolation, DiscordError, Error, Result, ToResult as _,
    };
    pub(crate) use crate::user::User;
    pub(crate) use crate::utils::simple_callback;
    pub(crate) use discord_game_sdk_sys as sys;
    pub(crate) use std::os::raw::{c_char, c_void};
}

pub use create_flags::CreateFlags;
pub use discord::Discord;
pub use error::{Error, Result};
pub use premium_type::PremiumType;
pub use user::User;
