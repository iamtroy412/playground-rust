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

    let mut x = 10;
    let r1 = &x;
    let r2 = &x; // ok; multiple shared borrows allowed
    x += 10; // error; cannot assign to 'x' because it's borrowed.
    let m = &mut x; // error; cannot borrow 'x' as mutable because it is
                    // also borrowed as immutable.
    println!("{}, {}, {}", r1, r2, m); // The references are used here,
                                       // so their lifetimes must last
                                       // at least this long

    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y; // error; cannot borrow as mutable more than once
    let z = y; // error; cannot use 'y' because it was mutably borrowed
    println!("{}, {}, {}", m1, m2, z); // references are used here

    // It's okay to reborrow a shared reference from a shared reference;

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0; // ok, reborrowing shared as shared
    let m1 = &mut r.1; // error; can't reborrow shared as mutable
    println!("{}", r0); // r0 gets used here

    // You can reborrow from a mutable reference:

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; // ok, reborrowing mutable form mutable
    *m0 = 137;
    let r1 = &m.1; // ok, reborrowing shared from mutable
    v.1; // error access through other paths still forbidden
    println!("{}", r1); // r1 gets used here
}
