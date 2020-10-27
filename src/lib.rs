#![feature(specialization)]
#![allow(warnings)]

#[macro_use]
extern crate diesel;
extern crate byteorder;
extern crate libc;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate num;
#[cfg(test)]
#[macro_use]
extern crate num_derive;

pub mod oracle;

#[cfg(test)]
mod test;

#[cfg(test)]
mod logger;
