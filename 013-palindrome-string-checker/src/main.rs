fn remove_non_alphanumeric(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect()
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut forward: usize = 0;
    let mut backward: usize = len - 1;

    while forward < backward {
        if chars[forward] != chars[backward] {
            return false;
        }
        forward += 1;
        backward -= 1;
    }

    true
}

fn main() {
    let input_string: &str = "A man, a plan, a canal, Panama!";
    let cleaned_string = remove_non_alphanumeric(input_string.to_lowercase().as_str());
    if is_palindrome(&cleaned_string) {
        println!("True: String is palindrome.");
    } else {
        println!("False: String is not palindrome.");
    }
}
