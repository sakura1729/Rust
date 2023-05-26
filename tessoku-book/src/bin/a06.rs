use proconio::{fastout, input};
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut n: usize,
        q: usize,
        A: [usize;n],
        V: [[usize;2];q],
    }
    let mut S = vec![0; n + 1];
    for i in 0..n {
        S[i + 1] = S[i] + A[i];
    }
    for i in 0..q {
        println!("{}", S[V[i][1]] - S[V[i][0] - 1]);
    }
}
