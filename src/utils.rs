use std::io;

pub fn read_from_stdin(msg: &mut String, error_msg: &str) {
    io::stdin().read_line(msg).expect(error_msg);
}
