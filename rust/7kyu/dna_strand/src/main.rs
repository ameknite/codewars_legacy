fn main() {}

fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|x| match x {
            'A' => 'T',
            'C' => 'G',
            'T' => 'A',
            'G' => 'C',
            _ => x,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_strand("AAAA"), "TTTT");
        assert_eq!(dna_strand("ATTGC"), "TAACG");
        assert_eq!(dna_strand("GTAT"), "CATA");
    }
}
