// I don't have the slightest idea how "109", the test won't pass, is supposed to be even valid
// outside any code. Raincheck.

// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code = code.to_string();
    let old_len = code
        .chars()
        .filter(|v| !v.is_whitespace())
        .collect::<Vec<_>>();

    for (i, c) in code.char_indices().filter(|v| !v.1.is_whitespace()) {
        if i > 0 && !c.is_numeric() {
            return false;
        }
    }
    code.retain(|v| v.is_numeric());

    let condition = code
        .char_indices()
        .scan(vec![None; code.len()], |state, x| {
            // unwrap is safe, we don't operate on any assumptions but rather we change the value
            let value = state.get_mut(x.0).unwrap();

            // this filters out the zeroes by making them true in the resulting vector but
            // otherwise making nonzero numbers false; this is then used in a call to filter as
            // this state vector is also the return value of the iterator for scan()
            // unwrap is safe, we know all chars in `code` at this point to be digits
            if x.1.to_digit(10).unwrap() == 0 {
                *value = Some(true);
            } else {
                *value = Some(false);
            }

            // this handles the case of consecutive zeroes, to keep them by making them the type of
            // values that will later be used to filter out (i.e. `false` to keep them, as nonzero
            // values use that to be kept and we want to keep these even though they are zeroes
            // because they are consecutive zeroes)
            // unwraps are safe because they act on variables that have already been assgined some
            // value or are otherwise known to store valid values (e.g. digits in the input)
            if x.0 > 0
                && x.1.to_digit(10).unwrap() == 0
                && code
                    .chars()
                    .collect::<Vec<_>>()
                    .get(x.0 - 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    == 0
            {
                *state.get_mut(x.0).unwrap() = Some(false);
                *state.get_mut(x.0 - 1).unwrap() = Some(false);
            }

            Some(state.clone())
        })
        .last()
        // unwrap is safe, the state is an always initialized vector, independent of the input
        .unwrap()
        .into_iter()
        .filter(|v| !v.unwrap())
        // at this point, we have only the nonzero digits when appropriate, e.g. in "059" we get 59
        // but in "0000 0" we get 00000); these are stored in the form of boolean values that
        // represent whether the value is zero or not
        .collect::<Vec<_>>()
        .len();
    if condition > 0 && condition % 2 == 0 && code.len() == old_len.len() {
        code = code
            .char_indices()
            .scan(String::new(), |state, v| {
                // it's not a zero; push it straight to prod
                if v.1.to_digit(10).unwrap() != 0 {
                    state.push(v.1);
                // it's a zero; it's not at the start; the one coming before it was also a zero
                } else if v.1.to_digit(10).unwrap() == 0
                    && v.0 > 0
                    && state
                        .chars()
                        .collect::<Vec<_>>()
                        .get(v.0 - 1)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                        == 0
                {
                    state.push(v.1);
                // it's a zero; it's at the start; the one coming after it is a zero as well
                } else if v.1.to_digit(10).unwrap() == 0
                    && v.0 == 0
                    && code
                        .chars()
                        .collect::<Vec<_>>()
                        .get(v.0 + 1)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                        == 0
                {
                    state.push(v.1);
                }

                Some(state.clone())
            })
            .last()
            .unwrap();
        // code.retain(|v| v.to_digit(10).unwrap() != 0);
    }

    if code.len() > 1
        && old_len.len()
            == old_len
                .iter()
                .filter(|v| v.is_ascii())
                .collect::<Vec<_>>()
                .len()
    {
        let mut doubled = vec![];

        for (_, c) in code.char_indices().map(|mut v| {
            if v.0 % 2 == 0 {
                // unwrap is safe; at this point, we know these numbers to only go from 0 to 9
                let output = v.1.to_digit(10).unwrap() * 2;

                v.1 = if output > 9 {
                    (output - 9).to_string().chars().last().unwrap()
                } else {
                    output.to_string().chars().last().unwrap()
                };
            }

            v
        }) {
            doubled.push(c);
        }

        // unwrap is safe; at this point, we know these numbers to only go from 0 to 9
        let doubled: Vec<u32> = doubled.iter().map(|v| v.to_digit(10).unwrap()).collect();

        let output: u32 = doubled.iter().sum();

        output % 10 == 0
    } else {
        false
    }
}
