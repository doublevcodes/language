use std::process;

use anyhow;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

#[derive(Debug, Clone)]
pub enum ErrorForm {
    SyntaxError,
    EOFError
}

#[derive(Debug, Clone)]
pub struct Errors {
    errors: Vec<Error>
}

impl Errors {
    pub fn new() -> Errors {
        Errors { errors: vec![] }
    }

    pub fn register_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn extend(&mut self, errors: &mut Vec<Error>) {
        self.errors.append(errors)
    }

    pub fn emit_errors(&mut self, file: SimpleFile<String, String>) -> Option<anyhow::Result<()>> {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = Config::default();

        if self.errors.len() == 0 {
            return None;
        }

        for mut error in self.errors.clone() {
            let error = error.as_diagnostic();
            term::emit(&mut writer.lock(), &config, &file, &error)
                .expect("Oh no");
        }

        process::exit(1)
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorForm,
    pub message: String,
    pub labels: Vec<Label<()>>,
    pub notes: Vec<String>,
}

impl Error {
    pub fn new(
        kind: ErrorForm,
        message: String,
        labels: Vec<Label<()>>,
        notes: Vec<String>,
    ) -> Error {
        Error {
            kind,
            message,
            labels,
            notes,
        }
    }

    pub fn as_diagnostic(&mut self) -> Diagnostic<()> {
        Diagnostic::error()
            .with_message(&self.message)
            .with_labels(self.labels.clone())
            .with_notes(self.notes.clone())
    }
}