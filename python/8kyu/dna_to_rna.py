import codewars_test as test

def dna_to_rna(dna: str) -> str:
    return dna.replace('T', 'U')

@test.describe("Sample Tests")
def basic_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(dna_to_rna("TTTT"), "UUUU")
        test.assert_equals(dna_to_rna("GCAT"), "GCAU")
        test.assert_equals(dna_to_rna("GACCGCCGCC"), "GACCGCCGCC")
