use crate::error::CliError;
use brief_diagnostic::FileId;
use brief_hir::{emit_bytecode, lower};
use brief_lexer::lex;
use brief_parser::parse;
use brief_runtime::Runtime;
use brief_vm::{VM, Value};
use rustyline::Context;
use rustyline::Helper;
use rustyline::Result as RustylineResult;
use rustyline::completion::{Completer, FilenameCompleter};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::Hinter;
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::validate::ValidationContext;
use rustyline::validate::ValidationResult;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, EditMode, Editor};

struct BriefHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
    validator: MatchingBracketValidator,
}

impl Completer for BriefHelper {
    type Candidate = <FilenameCompleter as Completer>::Candidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> RustylineResult<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for BriefHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for BriefHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
}

impl Helper for BriefHelper {}

impl Validator for BriefHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> RustylineResult<ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}

/// Run the REPL
pub fn repl() -> Result<(), CliError> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular) // Use Circular to allow tab insertion
        .edit_mode(EditMode::Emacs)
        .tab_stop(4) // 4 spaces per tab
        .build();

    let h = BriefHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::default(),
        hinter: HistoryHinter {},
        validator: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));

    let file_id = FileId(0);

    println!("Brief REPL");
    println!("Type 'exit' or 'quit' to exit, 'help' for help");
    println!("Press Enter to execute, or continue typing for multi-line input");
    println!("Tab inserts spaces for indentation");

    let mut vm = VM::new();
    let runtime = Runtime::new();
    vm.set_runtime(Box::new(runtime));

    loop {
        // Collect multi-line input
        let mut input = String::new();
        let mut is_first_line = true;

        loop {
            let prompt = if is_first_line { "brief> " } else { "      " };
            let readline = rl.readline(prompt);

            match readline {
                Ok(line) => {
                    // Check for special commands (only on first line)
                    if is_first_line {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if trimmed == "exit" || trimmed == "quit" {
                            return Ok(());
                        }
                        if trimmed == "help" {
                            println!("Commands:");
                            println!("  exit, quit - Exit the REPL");
                            println!("  help - Show this help message");
                            println!("Enter Brief code to evaluate");
                            println!("Press Enter on empty line to execute multi-line input");
                            continue;
                        }
                    }

                    // If line is empty and we have input, execute
                    if line.trim().is_empty() && !input.is_empty() {
                        break;
                    }

                    // Add line to input
                    if !input.is_empty() {
                        input.push('\n');
                    }
                    input.push_str(&line);
                    is_first_line = false;

                    // Check if input looks complete (heuristic: ends with newline or is a simple expression)
                    // For now, continue collecting until empty line
                }
                Err(ReadlineError::Interrupted) => {
                    if input.is_empty() {
                        println!("CTRL-C");
                        return Ok(());
                    } else {
                        // Clear multi-line input
                        input.clear();
                        is_first_line = true;
                        continue;
                    }
                }
                Err(ReadlineError::Eof) => {
                    if input.is_empty() {
                        println!("CTRL-D");
                        return Ok(());
                    } else {
                        // Execute what we have
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    return Err(CliError::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Readline error: {:?}", err),
                    )));
                }
            }
        }

        if input.trim().is_empty() {
            continue;
        }

        // Add to history
        let _ = rl.add_history_entry(input.as_str());

        // Wrap in a function for execution
        // The input may already be multi-line, so we need to indent each line
        let indented: String = input
            .lines()
            .map(|line| format!("\t{}", line))
            .collect::<Vec<_>>()
            .join("\n");
        let wrapped = format!("def __repl__()\n{}\n", indented);

        // Try to execute
        match execute_repl_line(&wrapped, file_id, &mut vm) {
            Ok(result) => {
                if let Some(value) = result {
                    if value != Value::Null {
                        println!("{}", value);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn execute_repl_line(
    source: &str,
    file_id: FileId,
    vm: &mut VM,
) -> Result<Option<brief_vm::Value>, CliError> {
    // 1. Lex
    let (tokens, lex_errors) = lex(source, file_id);
    if !lex_errors.is_empty() {
        eprintln!("Lexical errors:");
        for err in &lex_errors {
            eprintln!("  {:?}", err);
        }
        return Err(CliError::LexError);
    }

    // 2. Parse
    let (program, parse_errors) = parse(tokens, file_id);
    if !parse_errors.is_empty() {
        eprintln!("Parse errors:");
        for err in &parse_errors {
            eprintln!("  {:?}", err);
        }
        return Err(CliError::ParseError);
    }

    // 3. Lower to HIR
    let hir_program = match lower(program) {
        Ok(hir) => hir,
        Err(errors) => {
            eprintln!("HIR errors:");
            for err in &errors {
                eprintln!("  {:?}", err);
            }
            return Err(CliError::HirError(errors));
        }
    };

    // 4. Emit bytecode
    let chunks = emit_bytecode(&hir_program);

    if chunks.is_empty() {
        return Ok(None);
    }

    // 5. Execute
    use std::rc::Rc;
    let target_chunk = chunks
        .iter()
        .find(|chunk| chunk.name == "__repl__")
        .cloned()
        .unwrap_or_else(|| chunks[0].clone());
    let main_chunk = Rc::new(target_chunk);
    vm.push_frame(main_chunk, 0);

    // 6. Run VM
    match vm.run() {
        Ok(value) => Ok(Some(value)),
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            Err(CliError::RuntimeError(e))
        }
    }
}
