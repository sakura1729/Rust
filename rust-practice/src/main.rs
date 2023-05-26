use proconio::input;

fn main() {
    loop {
        input! {
            x: i32,
        }
        let y: i32 = match x {
            0 => break,
            x => x - 1,
        };
        println!("{}", y);
    }
}
