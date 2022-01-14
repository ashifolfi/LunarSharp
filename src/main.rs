#![allow(non_snake_case)]

macro_rules! check {
	($tocheck: expr) => {
		match $tocheck {
			Ok(t) => t,
			Err(e) => return Err(e.to_string())
		}
	};
}

mod scanner;
mod parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use scanner::Token;
use parser::ComplexToken;

fn CompileFile(path: &Path, name: String) -> Result<(), String> {
	let mut code: String = String::new();
	check!(check!(File::open(path)).read_to_string(&mut code));
	let tokens: Vec<Token> = scanner::ScanCode(code, name.clone())?;
	let ctokens: Vec<ComplexToken> = parser::ParseTokens(tokens, name.clone())?;
	/*let compiledname: String = String::from(path.display()
		.to_string()
		.strip_suffix(".clue")
		.unwrap()) + ".lua";
	let output: File = check!(File::create(compiledname));*/
	/*for token in tokens.iter() {
		println!("{} \"{}\" {}", token.kind, token.lexeme, token.line);
	}*/
	println!("{:#?}", ctokens);
	Ok(())
}

fn CompileFolder(path: &Path) -> Result<(), String> {
	for entry in check!(fs::read_dir(path)) {
		let entry = check!(entry);
		let name: String = entry.path().file_name().unwrap().to_string_lossy().into_owned();
		let filePathName: String = path.display().to_string() + "\\" + &name;
		let filepath: &Path = &Path::new(&filePathName);
		if filepath.is_dir() {
			CompileFolder(filepath)?;
		} else if filePathName.ends_with(".clue") {
			CompileFile(filepath, name)?;
		}
	}
	Ok(())
}

fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	let codepath: String;
	if args.len() == 1 {
		println!("Command use:\n\nclue [Path]/-version");
		return Ok(());
	}
	codepath = args[1].clone();
	if codepath == "-version" {
		println!("Version a1.0.18");
		return Ok(());
	}
	let path: &Path = Path::new(&codepath);
	if !path.is_dir() {
		return Err(String::from("The given path doesn't exist"));
	}
	CompileFolder(path)?;
	Ok(())
}