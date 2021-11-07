#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
use core::marker::PhantomData;
use core::ops::Deref;

#[doc = "Controller area network"]
pub mod fdcan;
pub mod generic;
