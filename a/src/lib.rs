#[cfg(feature = "foo")]
pub struct Foo;

#[cfg(feature = "bar")]
pub struct Bar;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
