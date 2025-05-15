pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut output = String::new();

    for bottle in (start_bottles - take_down + 1..=start_bottles).rev() {
        const HANGING: &str = "hanging on the wall,\n";
        const SINGLE: &str = "green bottle";
        const MULTI: &str = "green bottles";

        match bottle {
            1 => output.push_str(&format!(
                "One {} {}One {} {}",
                SINGLE, HANGING, SINGLE, HANGING
            )),
            2 => output.push_str(&format!(
                "Two {} {}Two {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            3 => output.push_str(&format!(
                "Three {} {}Three {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            4 => output.push_str(&format!(
                "Four {} {}Four {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            5 => output.push_str(&format!(
                "Five {} {}Five {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            6 => output.push_str(&format!(
                "Six {} {}Six {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            7 => output.push_str(&format!(
                "Seven {} {}Seven {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            8 => output.push_str(&format!(
                "Eight {} {}Eight {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            9 => output.push_str(&format!(
                "Nine {} {}Nine {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            10 => output.push_str(&format!(
                "Ten {} {}Ten {} {}",
                MULTI, HANGING, MULTI, HANGING
            )),
            _ => (),
        }

        output.push_str("And if one green bottle should accidentally fall,\n");
        output.push_str("There'll be ");
        match bottle {
            1 => output.push_str(&format!("no {}", MULTI)),
            2 => output.push_str(&format!("one {}", SINGLE)),
            3 => output.push_str(&format!("two {}", MULTI)),
            4 => output.push_str(&format!("three {}", MULTI)),
            5 => output.push_str(&format!("four {}", MULTI)),
            6 => output.push_str(&format!("five {}", MULTI)),
            7 => output.push_str(&format!("six {}", MULTI)),
            8 => output.push_str(&format!("seven {}", MULTI)),
            9 => output.push_str(&format!("eight {}", MULTI)),
            10 => output.push_str(&format!("nine {}", MULTI)),
            _ => (),
        }
        output.push_str(" hanging on the wall.\n\n");
    }

    output
}
