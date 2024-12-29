/*
125. Valid Palindrome

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters,
it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.
*/

pub fn is_palindrome(s: String) -> bool {
    // this leetcode probably assumes ascii which is not very
    // so we create an array of glyphs we can index into
    // this does allocation and copy but no way around it in the world of UTF-8
    let chars: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let length = chars.len();

    for i in 0..(length / 2) {
        if chars[i] != chars[length - i - 1] {
            return false;
        }
    }

    return true;
}
