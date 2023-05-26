use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut k = n;
    let mut answer = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in (0..10).rev() {
        answer[i] = k % 2;
        k = k / 2;
    }
    for x in answer.iter() {
        print!("{}", x);
    }
}
