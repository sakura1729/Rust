use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        A: [usize;n],
    }
    let mut answer = false;
    for i in A.iter() {
        if *i == x {
            answer = true;
        }
    }
    if answer == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
