use std::collections::HashMap;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Helper, Context};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableStore {
    variables: HashMap<String, String>,
}

impl VariableStore {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn all(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

pub type CommandResult = Result<String, String>;

pub struct PipelineCommand {
    pub name: String,
    pub args: Vec<String>,
}

pub enum CommandOutput {
    Text(String),
    None,
}

pub type CommandRegistry = HashMap<String, Box<dyn Fn(Vec<String>, &mut VariableStore) -> CommandResult + Send + Sync>>;

#[derive(Clone)]
pub struct ShellFlowCompleter {
    pub commands: Vec<String>,
}

impl Completer for ShellFlowCompleter {
    type Candidate = Pair;
    fn complete(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let matches: Vec<Pair> = self.commands
            .iter()
            .filter(|cmd| cmd.starts_with(line))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: cmd.clone(),
            })
            .collect();
        Ok((0, matches))
    }
}

pub struct ShellFlowHelper {
    pub completer: ShellFlowCompleter,
}

impl Completer for ShellFlowHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for ShellFlowHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for ShellFlowHelper {}

impl Validator for ShellFlowHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for ShellFlowHelper {}
