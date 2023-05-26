use proconio::{fastout, input};
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn binary_search(A: &[usize], k: usize) -> usize {
    let mut L: usize = 0;
    let mut R: usize = 1000000000;
    while L < R {
        let M: usize = (R + L) / 2;
        let mut sum: usize = 0;
        for i in A.iter() {
            sum += M / i;
        }
        if sum >= k {
            R = M;
        }
        if sum < k {
            L = M + 1;
        }
    }
    L
}
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        A: [usize;n],
    }
    println!("{}", binary_search(&A, k));
}
