use sys::{VALUE};
use super::{FromRuby, CheckResult, ToRuby, ToRubyResult};

impl FromRuby for VALUE {
    type Checked = VALUE;

    fn from_ruby(value: VALUE) -> CheckResult<VALUE> {
        Ok(value)
    }

    fn from_checked(checked: VALUE) -> VALUE {
        checked
    }
}

impl ToRuby for VALUE {
    fn to_ruby(self) -> ToRubyResult {
        Ok(self)
    }
}
