use std::io;

pub fn read_from_stdin(msg: &mut String, error_msg: &str) {
    io::stdin().read_line(msg).expect(error_msg);
}

pub fn get_values_by_delim(s: String, delim: &str) -> Vec<String> {
    s.split(delim)
        .filter_map(|val| {
            if val.is_empty() {
                return None;
            }
            Some(val.to_owned())
        })
        .collect::<Vec<String>>()
}
