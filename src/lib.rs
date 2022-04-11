/*
RSEC by Alexander Abraham,
a.k.a. "Angeldust Duke" a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

pub const SECURITY_WEIGHT: usize = 3;
pub const ARABIC_CHAR_WEIGHT: usize = 2;
pub const SPECIAL_CHAR_WEIGHT: usize = 3;
pub const SECURE_PASSWORD_SCORE: usize = 8;

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

/// Checks if the supplied string
/// can be converted to a number.
pub fn is_num(num: String) -> bool {
    let mut result: bool = false;
    let conv_op = num.parse::<usize>();
    match conv_op {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    }
    return result;
}

/// Converts a string to a number.
pub fn conv_string_to_usize(num: String) -> usize {
    let mut result: usize = 0;
    let num_clone_one: String = num.clone();
    let num_clone_two: String = num_clone_one.clone();
    if is_num(num_clone_one) {
        result = num_clone_two.parse::<usize>().unwrap();
    }
    else {}
    return result;
}

/// Returns the index of an item in a vector.
pub fn get_list_item_index(list: Vec<String>, item: String) -> usize {
    return list.iter().position(|r| r == &item).unwrap();
}

/// Returns the position of a character in the
/// alphabet.
pub fn char_pos(char: String) -> usize {
    let mut result: usize = 0;
    let char_clone_one: String = char.clone();
    let char_clone_two: String = char_clone_one.clone();
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphabet_clone_one = alphabet.clone();
    let alphabet_clone_two = alphabet_clone_one.clone();
    for letter in clean_split(alphabet_clone_one, String::from("")) {
        if &char_clone_one == &letter {
            result = clean_split(
                alphabet_clone_two.clone(), String::from("")
            ).iter().position(|r| r == &char_clone_two).unwrap();
        }
        else {}
    }
    return result;
}

/// Returns the distance between two characters.
pub fn get_char_space(char_one: String, char_two: String) -> usize {
    let char_one_clone_one = char_one.clone();
    let char_one_clone_two = char_one_clone_one.clone();
    let char_one_clone_three = char_one_clone_two.clone();
    let char_two_clone_one = char_two.clone();
    let char_two_clone_two = char_two_clone_one.clone();
    let char_two_clone_three = char_two_clone_two.clone();
    if char_pos(char_two_clone_one) < char_pos(char_one_clone_one){
        return char_pos(char_two_clone_two) + char_pos(char_one_clone_two);
    }
    else {
        return char_pos(char_two_clone_three) - char_pos(char_one_clone_three);
    }

}

/// Returns the space between two strings, that are numbers.
pub fn get_number_space(num_one: String, num_two: String) -> usize {
    let num_one_clone_one = num_one.clone();
    let num_one_clone_two = num_one_clone_one.clone();
    let num_one_clone_three = num_one_clone_two.clone();
    let num_two_clone_one = num_two.clone();
    let num_two_clone_two = num_two_clone_one.clone();
    let num_two_clone_three = num_two_clone_two.clone();
    if char_pos(num_two_clone_one) < char_pos(num_one_clone_one){
        return char_pos(num_two_clone_two) + char_pos(num_one_clone_two);
    }
    else {
        return char_pos(num_two_clone_three) - char_pos(num_one_clone_three);
    }
}

/// Returns the type of string: "special_char" or "letter".
pub fn string_type(char: String) -> String {
    let mut result: String = String::from("special_char");
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphabet_list: Vec<String> = clean_split(alphabet, String::from(""));
    for letter in alphabet_list {
        if letter == char {
            result = String::from("letter");
        }
    }
    return result;
}

/// Returns the type of character:
/// "special_char", "letter", or "num".
pub fn char_type(char: String) -> String {
    let char_clone_one: String = char.clone();
    let char_clone_two: String = char_clone_one.clone();
    let mut _result: String = String::from("");
    if is_num(char_clone_one) {
        _result = String::from("num");
    }
    else {
        _result = string_type(char_clone_two);
    }
    return _result;
}

/// Gives a score for a password.
pub fn password_strength(password: String) -> usize {
  let mut result: usize = 0;
  let char_list: Vec<String> = clean_split(password,String::from(""));
  let char_list_clone_one: Vec<String> = char_list.clone();
  let char_list_clone_two: Vec<String> = char_list_clone_one.clone();
  for char in char_list {
    let current_index: usize = get_list_item_index(char_list_clone_one.clone(), char);
    let current_item: String = char_list_clone_two[current_index].clone();
    let current_item_clone_one: String = current_item.clone();
    let current_item_clone_two: String = current_item_clone_one.clone();
    let current_item_type: String = char_type(current_item);
    let mut last_item: String = String::from("");
    if current_index == 0 {
        last_item = char_list_clone_two[current_index + 1].clone();
    }
    else {
        last_item = char_list_clone_two[current_index - 1].clone();
    }
    let last_item_clone_one: String = last_item.clone();
    let last_item_clone_two: String = last_item_clone_one.clone();
    let last_item_type: String = char_type(last_item);
    if current_item_type == "letter" && last_item_type == "letter" {
      let item_space: usize = get_char_space(current_item_clone_one, last_item_clone_two);
      if item_space > SECURITY_WEIGHT {
        result = result + ARABIC_CHAR_WEIGHT;
      }
      else {}
    }
    else if current_item_type == "special_char" && last_item_type == "special_char" {
      result = result + SPECIAL_CHAR_WEIGHT;
    }
    else if current_item_type == "num" && last_item_type == "num" {
        let item_space: usize = get_number_space(current_item_clone_two, last_item_clone_one);
      if item_space > SECURITY_WEIGHT {
        result = result + ARABIC_CHAR_WEIGHT;
      }
      else {}
    }
  }
  return result;
}

/// Returns a boolean depending on whether
/// the password is strong or not.
pub fn is_secure(password: String) -> bool {
    let mut result: bool = false;
    let password_rating: usize = password_strength(password);
    if password_rating > SECURE_PASSWORD_SCORE {
        result = true;
    }
    else {}
    return result;
}
