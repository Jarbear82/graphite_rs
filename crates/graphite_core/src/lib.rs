pub fn core_info() -> &'static str {
    "graphite_core v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_info() {
        assert_eq!(core_info(), "graphite_core v0.1.0");
    }
}
