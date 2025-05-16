pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors: Vec<_> = factors.iter().filter(|v| **v != 0).collect();

    let mut multiples = vec![];

    for i in factors {
        for n in 1..limit {
            if n % i == 0 {
                multiples.push(n);
            }
        }
    }

    multiples.sort_unstable();
    multiples.dedup();

    multiples.iter().sum()
}
