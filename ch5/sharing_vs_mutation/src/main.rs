fn main() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0]; // ok; vector is still there
    }
    let aside = v;
}
