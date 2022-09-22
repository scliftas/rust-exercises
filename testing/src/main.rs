#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn it_returns_hello_world() {
        assert_eq!(main(), "Hello, world!");
    }
}

fn main<'life>() -> &'life str {
    return "Hello, world!";
}
