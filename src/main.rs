mod challenge_27;
use challenge_27::remove_element;

mod challenge_125;
use challenge_125::is_palindrome;

fn main() {
    {
        println!("Challenge 27 : Remove Element");
        let mut tab = vec![0, 1, 2, 2, 3, 0, 4, 2];
        println!("Before : {tab:?}");
        let k = remove_element(&mut tab, 2);
        println!("Output : {k}, {tab:?}");
    }
    {
        println!("Challenge 125 : Valid Palindrome");
        println!(
            "\"A man, a plan, a canal: Panama\" : {}",
            is_palindrome("A man, a plan, a canal: Panama".to_owned())
        );
        println!(
            "\"race a car\" : {}",
            is_palindrome("race a car".to_owned())
        );
    }
}
