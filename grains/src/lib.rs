pub fn square(s: u32) -> u64 {
    if let 1..65 = s {
        2_u64.pow(s - 1)
    } else {
        panic!()
    }
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
