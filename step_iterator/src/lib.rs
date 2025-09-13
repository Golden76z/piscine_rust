#[derive(Debug, Clone)]
pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    finished: bool,
}

impl<T> StepIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + From<u8>, // for `0` and `1` conversions
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            finished: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // capture current value to return
        let val = self.current;

        // Check if next value would exceed end
        if val > self.end {
            self.finished = true;
            return None;
        }

        // prepare next current
        let next = val + self.step;

        if next > self.end {
            self.finished = true;
        } else {
            self.current = next;
        }

        Some(val)
    }
}
