#![cfg(feature = "cli")]
#![forbid(unsafe_code)]

use cargo_mobile2::{
	android::{NAME, cli::Input},
	util::cli::exec,
};

fn main() { exec::<Input>(NAME) }
