use mockall::automock;

#[automock]
trait Foo {
    fn foo(&self, x: u32, y: u32) -> u32;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::{Foo, MockFoo};
    use mockall::predicate::eq;

    #[test]
    fn test() {
        let mut mock = MockFoo::new();
        mock.expect_foo().with(eq(2), eq(3)).returning(|a, b| a * b);

        assert_eq!(6, mock.foo(2, 3));
    }
}
