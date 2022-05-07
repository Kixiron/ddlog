use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner, Rope};
use ddlog_lsp::{
    database::{DDlogDatabase, HirStore, Session, Source},
    Session as DDlogSession,
};
use ddlog_utils::Arc;
use std::io::{self, Write};

const REPL_FILE_NAME: &str = "repl/input.dl";

const HELP: &str = "
DDlog Repl

COMMANDS:
  :help            Show this message
  :item <item>     Parse the given item
  :expr <expr>     Parse the given expr
  :hir <item>      Output the hir for the given item
  :hir_expr <expr> Output the hir for the given expr
  :exit            Exit the repl
";

fn main() -> io::Result<()> {
    ddlog_driver::set_logger();

    let (stdin, mut stdout) = (io::stdin(), io::stdout());
    let mut input = String::new();

    let interner = Interner::new();
    let diagnostic_config = DiagnosticConfig::new();

    let mut file_cache = FileCache::new(interner.clone());
    let file = FileId::new(interner.get_or_intern_static(REPL_FILE_NAME));
    let mut database = DDlogDatabase::default();
    database.set_session(Arc::new(DDlogSession::new(interner.clone())));

    // TODO: Write a proper repl with rustyline
    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let input_str = input.trim();
        if input_str == ":exit" {
            println!("exiting...");
            break;
        } else if input_str == ":help" {
            println!("{}", HELP.trim());

            input.clear();
            continue;
        }

        let (prefix, command) = if input_str.starts_with(":expr ") {
            (":expr ", Command::ExprAst)
        } else if input_str.starts_with(":item ") {
            (":item ", Command::ItemAst)
        } else if input_str.starts_with(":hir_expr ") {
            (":hir_expr ", Command::ExprHir)
        } else if input_str.starts_with(":hir ") {
            (":hir", Command::ItemHir)
        } else {
            println!("invalid command");

            input.clear();
            continue;
        };

        database.set_file_source(file, Rope::from(input_str[prefix.len()..].trim()));

        let (parsed, diagnostics) = database.parsed(file);

        if matches!(command, Command::ExprAst | Command::ItemAst) {
            println!("{}", parsed.debug(&interner, true));
        }

        if !diagnostics.is_empty() {
            file_cache.add_str(file, input_str);

            for error in diagnostics.iter().cloned() {
                error
                    .emit_to(&diagnostic_config, &mut file_cache, &mut stdout)
                    .unwrap();
            }
        }

        if matches!(command, Command::ExprHir | Command::ItemHir) {
            let items = database.items(file);
            println!("{:#?}", items);
        }

        input.clear();
        file_cache.clear();
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    ExprAst,
    ItemAst,
    ItemHir,
    ExprHir,
}
