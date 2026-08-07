use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output = HashSet::from_iter([]);
    let mut temp_word: Vec<char> = word.to_lowercase().chars().collect();
    temp_word.sort_unstable();
    let sorted_word: String = temp_word.into_iter().collect();

    for item in possible_anagrams {
        if item.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut temp: Vec<char> = item.to_lowercase().chars().collect();
        temp.sort_unstable();
        let sorted: String = temp.into_iter().collect();

        if sorted == sorted_word {
            output.insert(*item);
        }
    }

    output
}
