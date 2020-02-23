pub fn of_u8(n: u8) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

pub fn of_u16(n: u16) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

pub fn of_u32(n: u32) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

pub fn of_u64(n: u64) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

pub fn of_u128(n: u128) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}
