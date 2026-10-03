fn intro() -> &'static str {
    // DONE: fix me 👇
    "I'm ready to start modelling a software ticket!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
