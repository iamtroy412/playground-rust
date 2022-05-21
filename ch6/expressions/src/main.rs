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

    // A let declaration can declare a variable with initializing it.
    // The variable can be initialized in a later assignment.
    // Can be usefule because sometimes the variable should be
    // initialized from some sort of control flow. Eithe way it will
    // only be initialized exactly once, so name does not need to
    // be declared as mut.
    let name;
    if user.has_nickname() {
        name = user.nickname();
    } else {
        name = generate_unique_name();
        user.register(&name);
    }

    // There is one more if form, the if let expression:
    if let pattern = expr {
        block1
    } else {
        block2
    }

    // The given expr either matches the pattern and block1 runs,
    // or it doesn't match and block2 runs. Sometimes this is a nice
    // way to get data out of an Option or Result:
    if let Some(cookie) = request.session_cookie {
        return restore_session(cookie);
    }

    if let Err(err) = show_cheesy_anti_robot_talk() {
        log_robot_attempt(err);
        politely_accuse_user_of_being_a_robot();
    } else {
        session.mark_as_human();
    }

    // An if let expression is shorthand for a match with just
    // one pattern:
    match expr {
        pattern => block1,
        _ => block2,
    }
}
