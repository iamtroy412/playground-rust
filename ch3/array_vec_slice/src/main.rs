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

}
