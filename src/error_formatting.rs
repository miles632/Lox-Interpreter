pub fn error_print(line: usize, msg: &str, wheree: &str) {

    eprintln!(
        "[line: {}], Error {}: where: {}",
        line,
        msg,
        wheree,
    );
}

