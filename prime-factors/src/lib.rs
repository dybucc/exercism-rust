pub fn factors(n: u64) -> Vec<u64> {
    let mut output = vec![];
    let mut n = n;
    let mut factor = 2;

    if n == 1 {
        return output;
    }

    loop {
        if n % factor == 0 {
            n /= factor;

            let mut multiples = 0;
            for i in 1..=factor {
                if factor % i == 0 {
                    multiples += 1;
                }
            }
            if multiples == 2 {
                output.push(factor);
            }

            if n == 1 {
                break;
            }
        } else {
            factor += 1;
        }
    }

    output
}
