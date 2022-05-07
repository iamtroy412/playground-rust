fn main() {
    let mut primes = vec![2, 3, 5, 7];
    let mut product: i32 = primes.iter().product::<i32>();
    assert_eq!(product, 210);
    println!("{}", product);

    primes.push(11);
    primes.push(13);

    product = primes.iter().product::<i32>();
    assert_eq!(product, 30030);
    println!("{}", product);

    // The vec! macro is the same thing as calling Vec::new() to create a new,
    // empty vector and then pushing the elements onto it.
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    // Another possibility is to build a vector from the values produced by
    // an iterator:
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // Same with arrays, you can use slice methods on vectors:

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    println!("{:?}", palindrome);
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    // Just as a vector's len method returns the number of elements it contains
    // now, the capacity method returns the number of elements it could hold
    // without realloction
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);

    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("Capacity is now {}", v.capacity());

    // You can insert and remove elements wherever you like, these
    // operations shift all the elements after the affected position
    // forward or backward, so may be slow if the vector is large.
    let mut v = vec![10, 20, 30, 40, 50];

    // Make the element at index 3 be 35
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    println!("{:?}", v);

    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
    println!("{:?}", v);

    // pop method removes last element and returns it. Technically,
    // an Option<T> is returned. None if vector is empty, or Some(v)
    // if the element had been v:
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    // You can use a for loop to iterator over a vector
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }

    // A slice [T] is a region of an array or vector. Slices can be any length,
    // slices can't be stored directly in variables or passed as function args.
    // Slices are always passed as references.
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    // Print a slice of numbers, one per line
    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }

    // Works on arrays and vectors
    print(&a);
    print(&v);

    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
    print(&sa[3..]);
}
