use std::f64:INFINITY;

fn held_karp(dist: &Vec<Vec<f64>>) -> (f64, Vec<size>)  {
    let n = dist.len();
    assert!(n >= 1);
    if(n == 1) {
        return (0.0, vec![0, 0]);
    }

    let full_mask =  (1usize << n) - 1;
    let size = 1usize << n;

    let mut dp = vec![INFINITY; size * n];
    let mut parent = vec![usize::MAX; size * n];

    let start_mask = 1usize << 0;
    dp[start_mask * n + 0] = 0.0;
    
}

fn main() {
    println!("Hello, world!");
}
