use derive_constructor::Constructor;

#[derive(Constructor, Debug)]
enum Foo {
    A,
    B(i32)
}

fn main() {
    print_foo(<_>::A);
    print_foo(<_>::B(42));
}

fn print_foo(foo: Foo) {
    println!("{:?}", foo);
}
