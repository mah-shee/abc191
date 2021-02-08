#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        v:usize,
        t:usize,
        s:usize,
        d:usize,
    }
    let point_sec = d as f64 / v as f64;
    if point_sec < t as f64 || point_sec > s as f64 {
        println!("Yes");
    } else {
        println!("No");
    }
}
