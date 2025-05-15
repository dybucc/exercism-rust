pub fn nth(n: u32) -> u32 {
    let mut multiples;
    let mut prime_count = u32::MAX;
    let mut output = 0;

    for num in 2.. {
        multiples = 0;

        for i in 1..=num {
            if num % i == 0 {
                multiples += 1;
            }
        }

        if multiples > 2 {
            continue;
        } else {
            prime_count = prime_count.wrapping_add(1);
        }

        if prime_count == n {
            output = num;
            break;
        }
    }

    output
}
