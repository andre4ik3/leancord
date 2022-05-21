pub fn etfpack() -> String {
    "etfpack".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(etfpack(), "etfpack".to_string());
    }
}
