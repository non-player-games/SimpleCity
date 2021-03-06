// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::guiddef::{IsEqualIID};
use shared::wtypes::{PROPERTYKEY, PROPID};
pub const PID_FIRST_USABLE: PROPID = 2;
pub type REFPROPERTYKEY = *const PROPERTYKEY;
#[inline]
pub fn IsEqualPropertyKey(a: &PROPERTYKEY, b: &PROPERTYKEY) -> bool {
    (a.pid == b.pid) && IsEqualIID(&a.fmtid, &b.fmtid)
}
