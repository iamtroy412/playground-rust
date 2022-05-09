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
}
