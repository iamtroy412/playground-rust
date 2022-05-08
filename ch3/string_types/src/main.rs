use regex::Regex;

fn main() {
    // String literals are enclosed in double quotes. They use the same
    // backslash escape sequence as char literals
    let speech = "\"Ouch!\" said the well.\n";

    // A string may be multiple lines.
    // The newline character in the string literal is included in the
    // string and therefore in the output.
    println!(
        "In the room the women come and go,
                Singing of Mount Abora"
    );

    // If one line of a string ends with a backslash, then the newline
    // character and the leading whitespace on the next line are
    // dropped.
    println!(
        "It was a bright, cold day in April, and \
        there were four of us-\
        more or less."
    );

    // Raw strings are tagged with a lowercase 'r'. All backslashes
    // and whitespace are included verbatim. No escapse sequences are
    // recognized.
    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*");

    // A byte string is a slice of u8 values - that is, bytes - not
    // Unicode text.
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}
