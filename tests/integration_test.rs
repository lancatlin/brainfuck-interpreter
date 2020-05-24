use std::io::{BufWriter, Cursor};
use brainfuck::{Interpreter, Error};

fn execute_from_string(program: String, input: String) -> Result<String, Error> {
    let program = Cursor::new(program.as_bytes());
    let input = Cursor::new(input.as_bytes());
    let mut output = BufWriter::new(Vec::new());
    let mut interpreter = Interpreter::new(program, input, &mut output);
    interpreter.execute()?;
    Ok(String::from_utf8(output.buffer().to_vec()).unwrap())
}

#[test]
fn test_io() {
    let code = String::from(",.");
    let input = String::from("a");
    assert_eq!("a", execute_from_string(code, input).unwrap());
}

#[test]
fn test_plus() {
    let code = String::from(",++.");
    let input = String::from("a");
    assert_eq!("c", execute_from_string(code, input).unwrap());
}
#[test]
fn test_helloworld() {
    let code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
>++.>+.+++++++..+++.>++.<<+++++++++++++++.
>.+++.------.--------.>+.>."
        .to_string();
    assert_eq!(
        "Hello World!\n",
        execute_from_string(code, "".to_string()).unwrap()
    );
}

#[test]
fn test_upper_case() {
    let code = ",----------[----------------------.,----------]".to_string();
    let input = "wancat\n".to_string();
    assert_eq!("WANCAT", execute_from_string(code, input).unwrap());
}

#[test]
fn test_double_loop() {
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
    let input = "".to_string();
    assert_eq!("Hello World!\n", execute_from_string(code, input).unwrap());
}
