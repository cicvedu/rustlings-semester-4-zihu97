// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    //input.to_string() + " world!"

    //let mut i = input.to_string().clone();
    //i.push_str(&String::from(" world!"));
    //i
    
    let mut i = input.to_string();
    i.push_str(" world!");
    i
}

fn replace_me(input: &str) -> String {
    let words : Vec<&str> = input.split_whitespace().collect();
    let mut s = String::new();
    for (index, word) in words.iter().enumerate() {
        if *word == "cars" {
            s += "balloons";
        } else {
            s += word;
        }
        if index != words.len() - 1 {
            s += " ";
        }
    }
    s
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
