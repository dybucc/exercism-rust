pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();
    let mut iter = list.iter().peekable();

    if list.is_empty() {
        return "".to_string();
    }

    loop {
        let mut dummy = "";

        match iter.next() {
            Some(v) if v == list.last().unwrap() => {
                output.push_str(&format!(
                    "And all for the want of a {}.",
                    list.first().unwrap()
                ));
                break;
            }
            Some(v) => dummy = v,
            None => (),
        };

        output.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            dummy,
            iter.peek().unwrap()
        ));
    }

    output
}
