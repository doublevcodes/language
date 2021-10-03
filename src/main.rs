use codespan_reporting::files::SimpleFile;
use language::parser::lexer::*;
use language::parser::token::Token;

fn main() {
    fn parse(input: &str) -> Vec<Token> {
        let file = SimpleFile::new(
            String::from("main.language"),
            String::from(input),
        );
        let mut lexer = Lexer::from_text(input, file);
        lexer.tokenise();
        let nfile = SimpleFile::new(
            String::from("main.language"),
            String::from(input),
        );
        lexer.error.emit_errors(nfile);
        lexer.tokens
    }

    let statement = parse(
        "
1 + \"str\" + 34567.8
a = 45 + 222 - \"6yh645bh65v58\"",
    );

    println!("{:?}", statement);
}