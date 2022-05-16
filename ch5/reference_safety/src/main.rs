fn main() {
    unsafe {
        println!("{}", STASH);
        f(&WORTH_POINTING_AT);
        println!("{}", STASH);
    }

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10);
}

struct S {
    r: &i32,
}

static mut STASH: &i32 = &42;
static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}
