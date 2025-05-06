
#![feature(specialization)]
//#![feature(min_specialization)]

// Allows macro expansion of `use ::solana_frozen_abi::*` to work within this crate
extern crate self as solana_frozen_abi;

pub mod abi_digester;
pub mod abi_example;
mod hash;

#[macro_use]
extern crate solana_frozen_abi_macro;

//#[macro_use]
//extern crate serde_derive;
