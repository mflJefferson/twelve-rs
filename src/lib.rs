pub fn calculate_percentage_change(first: f32, second: f32) -> f32 {
    if first < second {
        return (second - first) / first * 100.0;
    }
    if first > second {
        return (first - second) / first * 100.0 * -1.0;
    }

    return 0 as f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(1 as f32, 2 as f32), 100.0);
        assert_eq!(calculate_percentage_change(2 as f32, 1 as f32), -50.0);
        assert_eq!(calculate_percentage_change(1 as f32, 1 as f32), 0.0);
        assert_eq!(calculate_percentage_change(1.2, 2.4), 100.0);
        assert_eq!(calculate_percentage_change(2.4, 1.2), -50.0);
        assert_eq!(calculate_percentage_change(1.1, 1.1), 0.0);
        assert_eq!(calculate_percentage_change(0.06837900, 0.06642600), -2.8561375);
    }
}