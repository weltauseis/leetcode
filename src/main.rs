mod challenge_27;
use challenge_27::remove_element;

fn main() {
    {
        println!("Challenge 27 : Remove Element");
        let mut tab = vec![0, 1, 2, 2, 3, 0, 4, 2];
        println!("Before : {tab:?}");
        let k = remove_element(&mut tab, 2);
        println!("Output : {k}, {tab:?}");
    }
}
