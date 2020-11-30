use crate::fixed_queue::FixedQueue;
use crate::traits::Indicator;

pub struct Min {
    period: u32,
    history: FixedQueue<f64>,

}

impl Min {
    pub fn new(period: u32) -> Min {
        Self {
            period,
            history: FixedQueue::new(period),
        }
    }

    fn get_min(&self) -> f64 {
        let mut min = self.history.at(0).unwrap();
        for i in 0..self.history.size() as i32 {
            let elem = self.history.at(i).unwrap();
            if elem < min {
                min = elem;
            }
        }
        return min;
    }
}

impl Indicator<f64, Option<f64>> for Min {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.history.add(input);
        if self.history.is_full() {
            return Some(self.get_min());
        } else {
            return None;
        }
    }

    fn reset(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Indicator;
    use crate::math::min::Min;

    #[test]
    fn min_works() {
        let mut min = Min::new(5);
        assert_eq!(min.next(81.59), None);
        assert_eq!(min.next(81.06), None);
        assert_eq!(min.next(82.87), None);
        assert_eq!(min.next(83.00), None);
        assert_eq!(min.next(83.61), Some(81.06));
        assert_eq!(min.next(83.15), Some(81.06));
        assert_eq!(min.next(82.84), Some(82.84));
        assert_eq!(min.next(83.99), Some(82.84));
        assert_eq!(min.next(84.55), Some(82.84));
        assert_eq!(min.next(84.36), Some(82.84));
        assert_eq!(min.next(85.53), Some(82.84));
        assert_eq!(min.next(86.54), Some(83.99));
        assert_eq!(min.next(86.89), Some(84.36));
        assert_eq!(min.next(87.77), Some(84.36));
        assert_eq!(min.next(87.29), Some(85.53));
    }
}