mod bytecode;
mod lex;
mod parse;
mod value;
mod vm;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::vm::ExeState;

    use super::*;

    #[test]
    fn it_works() {
        let file = File::open("hello.lua").unwrap();
        let proto = parse::load(file);
        ExeState::new().execute(&proto);
    }
}
