fn main() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0]; // ok; vector is still there
    }
    let aside = v;

    fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
        for ele in slice {
            vec.push(*ele);
        }
    }

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head); // extend wave with another vector
    extend(&mut wave, &tail); // extend wave with an array

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // Doesn't work! pg 125. When extending &mut wave, it needs to
    // allocate a larger buffer, but &wave is still pointing to the
    // old buffer, which has been dropped by allocating the new one
    // for extending. The Rust compiler catches this with a borrow
    // error; cannot borrow mut and and immutable ref at the same time.
    // ie one read/write or many readers.
    extend(&mut wave, &wave);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
}
