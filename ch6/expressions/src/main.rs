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

    // There are 4 looping expressions:
    while condition {
        block
    }

    while let pattern = expr {
        block
    }

    loop {
        block
    }

    for pattern in iterable {
        block
    }

    // A for loop evaluates the iterable expression and then evaluates
    // the block once for each value in the resulting iterator.
    for i in 0..20 {
        println!("{}", i);
    }

    // In keeping with Rust's move semantics, a for loop over a value
    // consumes the value:
    let strings: Vec<String> = error_messages();
    for s in strings {
        // each String is moved into 's' here...
        println!("{}", s);
    } // ... and dropped here.
    println!("{} error(s)", strings.len()); // error: use of moved value

    // This can be inconvenient. The easy fix is to loop over a reference
    // to the collection instead. The loop variable then, will be a reference
    // to each item in the collection:
    for rs in &strings {
        println!("String {:? is at address {:p}.", *rs, rs);
    }
    // Here the type of &strings is &Vev<String>, and the type of
    // rs is &String.

    // Iterating over a mut refernce provides a mut reference to each element.
    for rs in &mut strings {
        // the type of rs is &mut String
        rs.push('\n'); // add a newline to each string
    }

    // A break expression exits an enclosing loop. Withinin the body of a loop,
    // you can give break an expression, whose value becomes that of the loop.
    // Naturally, all the break expressions within a loop must produce values
    // with the same type, which becomes the type of the loop itself.

    // Each call to 'next_line' returns either 'Some(line)', where 'line'
    // is a line of input, or 'None', if we've reached the end of the input.
    // Return the first line that starts with "answer: ".
    // Otherwise, return "answer: nothing".
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };

    // A continue expression jumps to the next loop iteration:
    for line in input_lines {
        let trimmed = trim_comments_and_whitespace(line);
        if trimmed.is_empty() {
            // Jump back to the top of the loop and
            // move on to the next line of input.
            continue;
        }
    }

    // A loop can be labeled with a lifetime. In the following example,
    // 'search: is a label for the outter loop. Thus, break 'search
    // exits that loop, not the inner loop:
    'search: for room in apartment {
        for spot in room.hiding_spots() {
            if spot.contains(keys) {
                println!("Your keys are {} in the {}.", spot, room);
                break 'search;
            }
        }
    }

    // A break can have both a label and a value expression:
    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let square = i * i;
            if square == n {
                break 'outer i;
            }
            if square > n {
                break;
            }
        }
    };
    // Labels can also be used with continue.
}
