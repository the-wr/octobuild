use std::io::IoError;
use std::io::IoErrorKind;
use std::io::Command;
use std::io::process::ProcessOutput;

// Scope of command line argument.
#[derive(Copy)]
#[derive(Show)]
#[derive(PartialEq)]
pub enum Scope {
	// Preprocessing argument
	Preprocessor,
	// Compiler argument
	Compiler,
	// Preprocessor & compiler argument
	Shared,
	// Unknown argument - local build only
	Ignore,
}

#[derive(Copy)]
#[derive(Show)]
#[derive(PartialEq)]
pub enum InputKind {
	Source,
	Marker,
	Precompiled,
}

#[derive(Copy)]
#[derive(Show)]
#[derive(PartialEq)]
pub enum OutputKind {
	Object,
	Marker,
}

#[derive(Show)]
#[derive(PartialEq)]
pub enum Arg {
	Flag{scope:Scope, flag: String},
	Param{scope:Scope, flag: String, value: String},
	Input{kind:InputKind, flag: String, file: String},
	Output{kind:OutputKind, flag: String, file: String}
}

pub struct CompilationTask {
	// Original compiler executable.
	pub command: Command,
	// Parsed arguments.
	pub args: Vec<Arg>,
	// Source language.
	pub language: String,
	// Input source file name.
	pub input_source: Path,
	// Input precompiled header file name.
	pub input_precompiled: Option<Path>,
	// Output object file name.
	pub output_object: Path,
	// Output precompiled header file name.
	pub output_precompiled: Option<Path>,
	// Marker for precompiled header.
	pub marker_precompiled: Option<String>,
}

pub struct PreprocessResult {
	// Hash
	pub hash: String,
	// Preprocessed file
	pub content: Vec<u8>,
}

pub trait Compiler {
	// Parse compiler arguments.
	fn create_task(&self, command: &Command, args: &[String]) -> Result<CompilationTask, String>;

	// Preprocessing source file.
	fn preprocess_step(&self, task: &CompilationTask) -> Result<PreprocessResult, IoError>;

	// Compile preprocessed file.
	fn compile_step(&self, task: &CompilationTask, preprocessed: PreprocessResult) -> Result<ProcessOutput, IoError>;

  // Run preprocess and compile.
	fn try_compile(&self, command: &Command, args: &[String]) -> Result<ProcessOutput, IoError> {
		match self.create_task(command, args) {
			Ok(task) => self.compile_step(&task, try! (self.preprocess_step(&task))),
			Err(e) => Err(IoError {
				kind: IoErrorKind::InvalidInput,
				desc: "Can't parse command line arguments",
				detail: Some(e)
			})
		}
	}

  // Run preprocess and compile.
	fn compile(&self, command: &Command, args: &[String]) -> Result<ProcessOutput, IoError> {
		match self.try_compile(command, args) {
			Ok(output) => Ok(output),
			// todo: log error reason
			Err(_) => command.clone().args(args).output()
		}
	}
}
