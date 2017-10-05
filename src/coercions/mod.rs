mod value;
mod unit;
mod bool;
mod integers;
mod float;
mod string;
mod vec;
mod option;
mod result;

use sys::{VALUE};
use super::{Error, ToError};
use std::marker::{PhantomData, Sized};

pub trait FromRuby : Sized {
    type Checked /* = CheckedValue<Self> */;

    fn from_ruby(value: VALUE) -> CheckResult<Self::Checked>;
    fn from_checked(checked: Self::Checked) -> Self;

    fn from_ruby_unwrap(value: VALUE) -> Self {
        Self::from_checked(Self::from_ruby(value).unwrap())
    }
}

pub type CheckResult<T> = Result<T, Error>;

pub struct CheckedValue<T> {
    inner: VALUE,
    marker: PhantomData<T>,
}

impl<T> CheckedValue<T> {
    pub unsafe fn new(inner: VALUE) -> CheckedValue<T> {
        CheckedValue { inner: inner, marker: PhantomData }
    }

    pub fn to_value(self) -> VALUE {
        self.inner
    }
}

pub type ToRubyResult = Result<VALUE, Error>;

pub trait ToRuby {
    fn to_ruby(self) -> ToRubyResult;
}

