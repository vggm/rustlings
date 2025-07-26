// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {

    use super::Command;

    fn make_uppercase(wrd: String) -> String {
        wrd.to_uppercase()
    }
 
    fn make_trim(wrd: String) -> String {
        wrd.trim().to_string()
    }

    fn make_append(wrd: String, times: usize) -> String {
        let mut result: String = wrd;

        for _ in 0..times {
            result += "bar";
        }

        result
    }

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for (word, comm) in input {
            match comm {
                Command::Trim                 => {result.push(make_trim(word))},
                Command::Uppercase            => {result.push(make_uppercase(word))},
                Command::Append(times) => {result.push(make_append(word, times))},
            }            
        }

        result
    }
}

fn main() {
    // You can optionally experiment here.
    use my_module::transformer;
    use Command;
    let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
    let output = transformer(input);
    println!("Debug: {output:?}");
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
