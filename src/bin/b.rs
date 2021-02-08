#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }
    let mut ans = vec![];
    for i in 0..n {
        if a[i] != x {
            ans.push(a[i]);
        }
    }
    for i in ans.iter() {
        print!("{} ", i);
    }
}
