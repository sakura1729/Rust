use proconio::{fastout, input};
#[fastout]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
    input! {
        d: usize,
        n: usize,
        A: [[usize;2];n]
    }
    let t = &d;
    let mut S = vec![0; d + 2];
    for i in 0..n {
        S[A[i][0]] += 1;
        S[A[i][1] + 1] -= 1;
    }
    for i in 0..*t {
        S[i + 1] += S[i]
    }
    for i in 1..=*t {
        println!("{}", S[i]);
    }
}
