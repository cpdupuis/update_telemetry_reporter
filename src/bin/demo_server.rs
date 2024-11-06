fn main() {
    println!("This is going to be a demo server eventually");
}

fn hello_there() -> &'static str {
    "Greetings"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_data() {
        let greeting = hello_there();
        assert_eq!("Greetings", greeting);
    }
}