fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a strings
    let mut start_index: usize = 0;
    let mut end_spaces: usize = 0;

    for c in input.chars() {
        if c != ' ' {break;}
        start_index += 1;
    }

    for c in input.chars().rev() {
        if c != ' ' {break;}
        end_spaces += 1;
    }

    let end_index = input.len()-end_spaces;

    {
        assert_eq!(input.trim(), &input[start_index..end_index]);
    }

    &input[start_index..end_index]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    
    {
        let string_insert = input.to_string() + " world!";
        let format_string = format!("{input} world!");

        assert_eq!(string_insert, format_string);
    }
    
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.to_string().replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
