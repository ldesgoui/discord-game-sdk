use crate::{
    event,
    panic_messages::{NULL_PTR, SEND_FAIL},
};
use std::ffi::c_void;

pub(crate) extern "C" fn on_current_user_update(senders: *mut c_void) {
    prevent_unwind!();

    unsafe { (senders as *mut event::Senders).as_ref() }
        .expect(NULL_PTR)
        .current_user_update
        .try_send(event::users::CurrentUserUpdate)
        .expect(SEND_FAIL)
}
