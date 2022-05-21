fn main() {
    // if and match statements can produce values

    // An if expression can be used to initialize a variable
    let status = if cpu.temperature <= MAX_TEMP {
        HttpStatus::Ok
    } else {
        HttpStatus::ServerError
    };

    // Since match also produes a value, it can be passed as an
    // argument to a function or a macro.
    println!(
        "Inside the vat, you see {}.",
        match vat.contents {
            Some(brain) => barin.desc(),
            None => "nothing of interest",
        }
    );

    // A block produces a value and can be used anywhere a value
    // is needed:
    let display_name = match post.author() {
        Some(author) => author.name(),
        None => {
            let network_info = post.get_network_metadata()?;
            let ip = network_info.client_address();
            ip.to_string()
        }
    };

    // When you leave the semicolon off the last line of a block,
    // that makes the value of the block the value of the final
    // expression, rather than the usual ().
    let msg = {
        // let-declaration: semicolon is alsways required
        let dandelion_control = puffball.open();

        // expression + semicolon: method is called, return value dropped
        dandelion_control.release_all_seeds(launch_codes);

        // expression with no semicolon: method is called,
        // return value stored in 'msg'.
        dandelion_control.get_status()
    };
}
