# derive-constructor

This crate provides a derive macro `Constructor`, which creates a silly trait that mirrors an enum's constructors. It allows constructing an enum without explicitly naming its type.
The generated trait has the same name as the enum but with the suffix `Constructor`.

## Example

```rust
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
```

