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

    // The to_string() method converts a &str to a String.
    let error_message = "too many pets".to_string();

    // The format!() macro works like println!() except it doesn't
    // print to stdout, it creates a String without the newline.
    assert_eq!(
        format!("{}d{:02}'{:02}'N", 24, 5, 23),
        "24d05'23'N".to_string()
    );

    // Arrays, slices, and vectors of string have two methods.
    // concat() and join(sep) that form new a new Sting
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // Strings support == and != operators. Two strings are equal if they
    // contain the same characters in the same order.
    assert!("ONE".to_lowercase() == "one");
}
