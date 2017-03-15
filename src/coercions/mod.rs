mod slice;
mod string;
mod unit;
mod bool;
mod integers;
mod option;

use sys::{VALUE};
use std::ffi::CString;
use std::marker::PhantomData;

pub struct CheckedValue<T> {
    pub inner: VALUE,
    marker: PhantomData<T>
}

impl<T> CheckedValue<T> {
    pub unsafe fn new(inner: VALUE) -> CheckedValue<T> {
        CheckedValue { inner: inner, marker: PhantomData }
    }
}

pub type CheckResult<T> = Result<CheckedValue<T>, CString>;

pub trait UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<T>;
}

pub trait ToRust<T> {
    fn to_rust(self) -> T;
}

pub trait ToRuby {
    fn to_ruby(self) -> VALUE;
}

impl ToRuby for VALUE {
    fn to_ruby(self) -> VALUE {
        self
    }
}