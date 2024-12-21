fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        return None; // 如果小时大于23，返回 None
    }

    if hour_of_day < 22 {
        Some(5) // 在22点之前，返回5个冰激凌
    } else {
        Some(0) // 在22点之后，返回0个冰激凌
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12).unwrap(); // 使用 unwrap 获取 Option 中的值

        assert_eq!(icecreams, 5); // 不改变这一行
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}