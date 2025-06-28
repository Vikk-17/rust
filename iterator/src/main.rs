/*
 *  A program to:
 *  - filter out all the odds
 *  - double each values
 *  - create a vector
 */

fn filter_and_map(v: Vec<u16>) -> Vec<u16> {
    let iter = v.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);
    // colllect all the value and make it vector
    let ans: Vec<u16> = iter.collect();
    ans
}

fn main() {
    let v1: Vec<u16> = vec![10, 11, 12, 13, 14, 15];
    // let v1_iter = v1.iter();
    // let odds = v1_iter.filter(|x| *x % 2 == 1).map(|x| x * 2);
    // println!("{:?}", odds);
    // println!("{:?}", v1);
    //
    // for i in odds {
    //     println!("{}", i);
    // }
    let ans = filter_and_map(v1);
    println!("{:?}", ans);
}
