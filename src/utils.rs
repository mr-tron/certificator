use std::string::ToString;

// todo: add colons between bytes
pub fn thumprint_repr(slice: &[u8]) -> String {
    let mut buf = "".to_string();
    for b in slice {
        fn hex_from_digit(num: u8) -> char {
            if num < 10 {
                (b'0' + num) as char
            } else {
                (b'A' + num - 10) as char
            }
        }
        buf.push(hex_from_digit(b / 16));
        buf.push(hex_from_digit(b % 16));
    }
    return buf;
}