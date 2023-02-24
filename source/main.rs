use std::env::args;
use std::fs;
use std::io::{stdin, stdout};
use std::time::Instant;
use yansi::Paint;

mod vm;

fn main() {
    let args: Vec<String> = args().collect();
    let stdio = stdin();
    let mut input = stdio.lock();
    let mut output = stdout();

    let instant = Instant::now();

    let source_code = fs::read_to_string(&args[1]).unwrap();
    let bf_instructions = vm::lexer(&source_code);
    let vm_instructions = vm::parse(&bf_instructions);

    vm::run(&vm_instructions, &mut input, &mut output);

    println!(
        "\n  {} {} {:.3?}",
        Paint::green(&args[1]).bold(),
        Paint::white("run in").dimmed(),
        Paint::cyan(instant.elapsed()).italic()
    )
}
