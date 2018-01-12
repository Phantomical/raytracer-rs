
use pest::prelude::*;

impl_rdp! {
	grammar! {
		file = _{ version ~ line* ~ eoi }

		integer = @{ ['0'..'9']+ }
		float   = @{ ['0'..'9']+ ~ ["."] ~ ['0'..'9']+ }
		ident   = @{ (["_"] | ['a'..'z'] | ['A'..'Z'])+ }
		number  = @{ ["-"]? ~ (integer | float) }
		vector  = { ["["] ~ (number ~ [","])+ ~ ["]"] }

		version = { ["version"] ~ ["0.1.0"] }
		line    = _{ object ~ [";"] }

		expression = _{ number | object | vector }
		field   = { ident ~ [":"] ~ expression }
		object  = { ident ~ ["{"] ~ ["m"] ~ ["}"]}

		whitespace = _{ [" "] | ["\r"] | ["\n"] | ["\t"] }
		comment    = _{ ["//"] ~ (!(["\n"] | ["\r"]) ~ any)* }
	}
}

pub fn parse_grammar(string : &str) {
	let mut parser = Rdp::new(StringInput::new(string));

	parser.file();
}
