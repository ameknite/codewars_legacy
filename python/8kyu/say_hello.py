import codewars_test as test


def say_hello(name: list[str], city: str, state: str) -> str:
    return f'Hello, {" ".join(name)}! Welcome to {city}, {state}!'


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(say_hello(['John', 'Smith'], 'Phoenix', 'Arizona'),
                           'Hello, John Smith! Welcome to Phoenix, Arizona!', 'Hello, John Smith! Welcome to Phoenix, Arizona!')
        test.assert_equals(say_hello(['Franklin', 'Delano', 'Roosevelt'], 'Chicago', 'Illinois'),
                           'Hello, Franklin Delano Roosevelt! Welcome to Chicago, Illinois!',)
        test.assert_equals(say_hello(['Wallace', 'Russel', 'Osbourne'], 'Albany',
                           'New York'), 'Hello, Wallace Russel Osbourne! Welcome to Albany, New York!')
        test.assert_equals(say_hello(['Lupin', 'the', 'Third'], 'Los Angeles', 'California'),
                           'Hello, Lupin the Third! Welcome to Los Angeles, California!')
        test.assert_equals(say_hello(['Marlo', 'Stanfield'], 'Baltimore', 'Maryland'),
                           'Hello, Marlo Stanfield! Welcome to Baltimore, Maryland!')
