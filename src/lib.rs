use rust2plc_macro::plc_fn;

#[plc_fn(
    st,
    description = "Adds two numbers",
    namespace = "Math",
    version = "1.0"
)]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
