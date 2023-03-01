use std::{env, fs::File, io::{BufReader, Read}};

use vm::VirtualMachine;

mod vm;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1])
            .expect(format!("can;t open '{}'", &args[1]).as_str());
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        let mut vm = VirtualMachine::new(&buf);
        vm.interpret();
    } else {
        eprintln!("Usage: brainfuck <FILE>");
    }
}
