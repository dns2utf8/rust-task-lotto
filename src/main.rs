/// Solved by Stefan Schindler <k11946678@students.jku.at>
use std::{env, num::ParseIntError, process::exit};

use rand::{prelude::SliceRandom, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        assert!(take <= from, "take must be smaller than from");
        let mut rng = thread_rng();
        let mut all_numbers = (0..from).collect::<Vec<usize>>();

        all_numbers.shuffle(&mut rng);

        let numbers = all_numbers[0..take].into();

        Self {
            take,
            from,
            numbers,
        }
    }

    // Need this function for testing only
    #[cfg(test)]
    fn get_numbers(self) -> Vec<usize> {
        self.numbers
    }
}

#[must_use]
fn format_lotto_results(lotto: &Lotto) -> String {
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() % 2 == 0, "expected an even amount of arguments");

    for pair in args.chunks(2) {
        if let Err(e) = do_lotto(&pair[0], &pair[1]) {
            eprintln!("invalid arguments: {:?} <- {:?}", pair, e);
            exit(1);
        }
    }
}

fn do_lotto(take: &String, from: &String) -> Result<(), ParseIntError> {
    let take = take.parse()?;
    let from = from.parse()?;

    let lotto = Lotto::new(take, from);

    println!("{}\n", format_lotto_results(&lotto));

    Ok(())
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
