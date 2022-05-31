fn main() {}

fn hor_mirror(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join("\n")
}

fn vert_mirror(s: String) -> String {
    s.split_whitespace()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn oper<F>(f: F, s: String) -> String
where
    F: Fn(String) -> String,
{
    f(s)
}

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(hor_mirror, s.to_string()), exp)
}
fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(vert_mirror, s.to_string()), exp)
}

#[test]
fn basics_oper() {
    testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
    testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
    testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

    testing2(
        "hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu",
        "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw",
    );
    testing2(
        "IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx",
        "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP",
    );
    testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
}
