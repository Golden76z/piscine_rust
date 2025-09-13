#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            return None;
        } else {
            let val = self.v;
            if self.v % 2 == 0 {
                self.v /= 2;
            } else {
                self.v = self.v * 3 + 1;
            }

            Some(Collatz { v: val })
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut steps = 0;
    let mut number = n;

    if n == 0 {
        return steps;
    }

    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = (number * 3) + 1;
        }

        steps += 1;
        // println!("Number: {}", number);
    }

    steps
}
