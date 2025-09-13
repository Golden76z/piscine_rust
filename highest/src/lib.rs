#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers: numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        } else {
            Some(self.numbers[self.numbers.len() - 1])
        }
    }

    pub fn highest(&self) -> Option<u32> {
        let mut biggest = 0;

        for i in 0..self.numbers.len() {
            if self.numbers[i] > biggest {
                biggest = self.numbers[i];
            }
        }

        if biggest == 0 { None } else { Some(biggest) }
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();
        let mut index = 0;

        // if self.numbers.len() < 3 {
        //     return vec;
        // }

        for i in 0..self.numbers.len() {
            vec.push(self.numbers[i]);

            if let Some((idx, _)) = vec.iter().enumerate().min_by_key(|&(_, v)| v)
                && index >= 3
            {
                vec.remove(idx);
            }

            index += 1;
        }

        vec.sort_by(|a, b| b.cmp(a));
        // println!("Vec: {:?}", vec);
        vec
    }
}
