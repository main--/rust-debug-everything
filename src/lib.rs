#![feature(specialization)]

//! Debug-print everything!
//!
//! If you - like me - frequently find yourself deep in a jungle of generic traits with
//! a sudden need to debug-print some data, this crate is for you.
//! The constant struggle of adding `Debug` bounds everywhere only to remove
//! all of them (or at least the ones you can still find) as soon as you're done is over:
//!
//! ```
//! use debug_everything::Debuggable;
//!
//! fn generic<T>(t: T) {
//!     println!("t = {:?}", t.debug());
//! }
//! ```
//!
//! # How it works
//!
//! Sadly, this relies on specialization and thus only works on nightly (as of May 2019).
//! The `Debuggable` trait is implemented for **all** types but *specialized* for types implementing
//! `Debug`:
//!
//! ```
//! use debug_everything::Debuggable;
//! struct Dummy;
//! 
//! assert_eq!("42", format!("{:?}", 42.debug()));
//! assert_eq!("<no Debug impl>", format!("{:?}", Dummy.debug()));
//! ```
//!
//! Simple!

use std::fmt::{Debug, Formatter, Result as FmtResult};

pub trait Debuggable {
    fn debug(&self) -> &dyn Debug;
}

struct NotDebug;
impl Debug for NotDebug {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str("<no Debug impl>")
    }
}

impl<T> Debuggable for T {
    default fn debug(&self) -> &dyn Debug { &NotDebug }
}

impl<T: Debug> Debuggable for T {
    fn debug(&self) -> &dyn Debug { self }
}

