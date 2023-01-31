/*
    TODO: company_directory() needs to be done, don't know enough yet.
    TODO: find_array_info(), I can't find mode with current knowledge, according to the Internet rustaceans.
*/

fn main() {
    let arr: [i32; 50] = [1,50,50,5,42,27,47,18,33,13,2,2,21,10,32,41,2,35,45,20,9,4,12,5,18,31,47,46,28,46,40,23,39,13,15,13,45,27,36,31,15,14,32,34,7,23,37,7,27,25];
    find_array_info(&arr);
    let phrase = "The apple is on the table";
    convert_to_pig_latin(phrase);
    company_directory();
}

fn convert_to_pig_latin(sentence: &str) {
    // Convert strings to pig latin. The first consonant of each word is moved to the
    // end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that
    // start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!

    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut new_sentence = String::new();

    for word in sentence.split_whitespace() {
        let char_vec: Vec<char> = word.chars().collect();

        let first_letter = char_vec[0];

        let new_word = if vowels.contains(&first_letter) {
            format!("{word}-hay ")
        } else {
            format!("{}-{}ay ", &word[1..], first_letter)
            // word[1..] + "-" + first_letter + "ay"
        };
        new_sentence.push_str(&new_word);
    }

    println!("Pig latin string: {}", new_sentence);
}

fn find_array_info(arr: &[i32]) {
    // Given a list of integers, use a vector and return the median (when sorted,
    //     the value in the middle position) and mode (the value that occurs most
    //     often; a hash map will be helpful here) of the list.

    println!("Working with the following array:\n{:?}", arr);

    // Find the median
    let mut v: Vec<i32> = Vec::new();

    v.extend_from_slice(&arr);
    v.sort();

    let middle = v.len() / 2;
    
    println!("The median is: {}.", v[middle]);

    // Don't think I know enough to do this yet.
    // Find the mode (most common value)
    // use std::collections::HashMap;

    // let mut map = HashMap::new();

    // for i in v {
    //     let count = map.entry(i).or_insert(0);
    //     *count += 1;
    // }

    // let max = 0;
    // let max_key = for (key, value) in &map {
    //     let max = if value > &max {
    //         value           
    //     } else {
    //         &max
    //     };
    //     key
    // };

    // println!("The number that appears the most is: {:?}.", max_key);
}

fn company_directory() {
    // Using a hash map and vectors, create a text interface to allow a user to add
    // employee names to a department in a company. For example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
    // department or all people in the company by department, sorted alphabetically.

    
}
