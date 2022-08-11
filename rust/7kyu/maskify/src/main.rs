fn main() {}

fn maskify(cc: &str) -> String {
    cc.char_indices()
        .map(|(i, c)| match c {
            _ if cc.len() - i > 4 => '#',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
