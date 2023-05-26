use proconio::{fastout, input};
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn binary_search(A: &[usize], x: usize) -> usize {
    let mut L: usize = 0;
    let mut R: usize = A.len() - 1;
    loop {
        let M: usize = (L + R) / 2;
        if A[M] < x {
            L = M + 1;
        }
        if A[M] > x {
            R = M - 1;
        }
        if A[M] == x {
            return M;
        }
    }
}
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        x: usize,
        A: [usize;n],
    }
    let a = binary_search(&A, x) + 1;
    println!("{}", a);
}
