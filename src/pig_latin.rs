// Given &str coming in, convert it to pig latin
// first consonant is moved to end  and ay added
// cup -> up-cay
// words that begin with a vowel just get hay added
// apple -> apple-hay
// this will be written for english only

const VOWELS: [char; 5] = ['e', 'u', 'i', 'o', 'a'];

fn convert_to_pig_latin(input: &str) -> String {
    // loop over every word
    // get first character of word
    // add suffix depending on first char
    // output new string
    let mut output = String::new();

    for word in input.split_whitespace() {
        let word_first_char: &char = &word.chars().next().expect("Not writing in edgecases");
        let mut word_pig_latin = String::new();

        if VOWELS.contains(word_first_char) {
            word_pig_latin = format!("{word}-hay");
        } else {
            word_pig_latin = word.chars().skip(1).collect::<String>()
                + "-"
                + &word_first_char.to_string()
                + "ay";
        }
        output += &word_pig_latin;
        output += " ";
    }
    output = output.trim().to_string();
    return output;
}

pub fn test_pig_latin() {
    let string_pre: String = String::from("Mary had a little");
    let string_pig = convert_to_pig_latin(&string_pre);

    assert_eq!(string_pig, "ary-May ad-hay a-hay ittle-lay");

    println!("Pig latin test passed successfully.")
}
