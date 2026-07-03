use rstu_parser::add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_also_here() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}