struct Closure<'a> {
    x: &'a mut i32,
}

impl Closure<'_> {
    fn call(&mut self) {
        *self.x += 1;
    }
}

fn main() {
    let mut x = 1;
    let mut y = || x = x + 1;
    y();

    println!("{}", x);
    let mut x = 1;
    let mut y = Closure { x: &mut x };
    y.call();
}
