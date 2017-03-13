pub fn raindrops(input: i64) -> String {
    let mut output = String::from("");

    if input % 3 == 0 {
        output += "Pling";
    }
    if input % 5 == 0 {
        output += "Plang";
    }
    if input % 7 == 0 {
        output += "Plong";
    }

    if output == "" {
        input.to_string()
    } else {
        output
    }
}
