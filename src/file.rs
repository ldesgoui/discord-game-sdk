use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileStat {
    pub filename: String,
    pub size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

impl FromSys for FileStat {
    type Source = sys::DiscordFileStat;

    fn from_sys(source: &Self::Source) -> Self {
        use chrono::offset::TimeZone;

        let filename = unsafe { std::ffi::CStr::from_ptr(&source.filename as *const _) }
            .to_str()
            .unwrap()
            .to_string();

        Self {
            filename,
            size: source.size,
            last_modified: chrono::Utc.timestamp(source.last_modified as i64, 0),
        }
    }
}