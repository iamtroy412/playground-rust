fn main() {
    // A variable owns its value. When control leaves the block in which the
    // variable is declared, the variable is dropped, so its value is
    // dropped along with it.
    fn print_padovan() {
        let mut padovan = vec![1, 1, 1]; // allocated here
        for i in 3..10 {
            let next = padovan[i - 3] + padovan[i - 2];
            padovan.push(next);
        }
        println!("P(1..10) = {:?}", padovan);
    } // dropped here

    // You can allocate a tuple in the heap like so
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    // Just as variables own their values, structs own their fields, and tuples,
    // arrays, and vectors own their elements.
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // Build a vector of string
    let mut v: Vec<String> = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // Pop a value off the end of the vetor:
    let last = v.pop().expect("vector empty!");
    assert_eq!(last, "105");

    // Move a value out of a given index in the vector,
    // and move the last element into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // Swap in another value for the one we're taking out
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // What's left in the vector?
    assert_eq!(v, vec!["101", "104", "substitute"]);
    println!("{:?}", v);

    // By default, user defined types like this struct are not copy(),
    // so calling print(l) would move ownership and the next println!()
    // would fail to compile. Using the below derive statement, we can
    // say that our struct has copy() and the below code compiles.
    // Note that this only works if your struct contains all types that
    // support copy()
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 42 };
    print(l);
    println!("My label number is: {}", l.number);

    // Hash table that maps String values to a vector of Vec<String>
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    fn show(table: &Table) {
        for (artist, works) in table {
            println!("works by {}:", artist);
            for work in works {
                println!(" {}", work);
            }
        }
    }

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }

    sort_works(&mut table);
    show(&table);
}
