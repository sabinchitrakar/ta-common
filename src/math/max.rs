use crate::ds::fixed_queue::FixedQueue;
use crate::traits::Indicator;

pub struct Max {
    period: u32,
    history: FixedQueue<f64>,

}

impl Max {
    pub fn new(period: u32) -> Max {
        Self {
            period,
            history: FixedQueue::new(period),
        }
    }

    fn get_max(&self) -> f64 {
        let mut max = self.history.at(0).unwrap();
        for i in 0..self.history.size() as i32 {
            let elem=self.history.at(i).unwrap();
            if elem > max {
                max = elem;
            }
        }
        return max;
    }
}

impl Indicator<f64, Option<f64>> for Max {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.history.add(input);
        if self.history.is_full() {
            return Some(self.get_max());
        }else{
            return None;
        }

    }

    fn reset(&mut self) {
        self.history.clear();
    }
}


#[cfg(test)]
mod tests {
    use crate::fixed_queue::FixedQueue;
    use crate::math::max::Max;
    use crate::traits::Indicator;

    #[test]
    fn max_works() {
        let mut max = Max::new(5);
        assert_eq!(max.next(81.59), None);
        assert_eq!(max.next(81.06), None);
        assert_eq!(max.next(82.87), None);
        assert_eq!(max.next(83.00), None);
        assert_eq!(max.next(83.61), Some(83.61));
        assert_eq!(max.next(83.15), Some(83.61));
        assert_eq!(max.next(82.84), Some(83.61));
        assert_eq!(max.next(83.99), Some(83.99));
        assert_eq!(max.next(84.55), Some(84.55));
        assert_eq!(max.next(84.36), Some(84.55));
        assert_eq!(max.next(85.53), Some(85.53));
        assert_eq!(max.next(86.54), Some(86.54));
        assert_eq!(max.next(86.89), Some(86.89));
        assert_eq!(max.next(87.77), Some(87.77));
        assert_eq!(max.next(87.29), Some(87.77));
    }
}