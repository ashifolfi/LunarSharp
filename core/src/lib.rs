#[cfg(feature = "rpmalloc")]
#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

pub mod compiler;
pub mod env;
pub mod parser;
pub mod preprocessor;
pub mod scanner;

#[macro_export]
macro_rules! check {
	($tocheck: expr) => {
		match $tocheck {
			Ok(t) => t,
			Err(e) => return Err(e.to_string()),
		}
	};
}

#[macro_export]
macro_rules! format_clue {
	($($strings:expr),+) => {{
		use std::ops::AddAssign;
		let mut len_format_clue = 0;
		$(len_format_clue.add_assign(AsRef::<str>::as_ref(&$strings).len());)+
		let mut output_format_clue = String::with_capacity(len_format_clue);
		$(output_format_clue.push_str($strings.as_ref());)+
		output_format_clue
	}};
}
