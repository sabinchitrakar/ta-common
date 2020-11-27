#[allow(dead_code)]
#[doc(include = "../README.md")]
#[derive(Debug,Clone)]
pub struct FixedQueue<T: Copy> {
    queue: Vec<T>,
    capacity: u32,
    index: u32,
}





/// # Examples
///
/// ```
/// use crate::ta_common::fixed_queue::FixedQueue;
/// let mut queue=FixedQueue::<i32>::new(2);
///  queue.add(2);
///  assert_eq!(1,queue.size());
///  queue.add(3);
///  assert_eq!(true,queue.is_full());
///  queue.add(4);
///  assert_eq!(2,queue.size());
///  queue.clear();
///  assert_eq!(0,queue.size());
/// ```
///
///

impl<T: Copy> FixedQueue<T> {
    #[allow(dead_code)]
    pub fn new(capacity: u32) -> FixedQueue<T> {
        Self {
            queue: Vec::<T>::with_capacity(capacity as usize),
            capacity,
            index: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, val: T) {
        if self.is_full() {
            self.queue.remove(0);
        }
        self.queue.push(val);
    }
    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        return self.capacity == self.queue.len() as u32;
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.queue = Vec::<T>::with_capacity(self.capacity as usize);
        self.index = 0;
    }
    pub fn reset(&mut self) {
        self.index = 0;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.queue.len();
    }

    pub fn at(&self, index: i32) -> Option<T> {
        if index < 0 || index > (self.queue.len() as i32 - 1i32) as i32 {
            return None;
        }
        let item = self.queue[index as usize];
        return Some(item);
    }
}

impl<T: Copy> Iterator for FixedQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.queue.len() as u32 {
            let res = self.at(self.index as u32 as i32);
            self.index = self.index + 1;
            return res;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed_queue::FixedQueue;

    #[test]
    fn fixed_queue_works() {
        let mut queue = FixedQueue::<i32>::new(2);
        queue.add(2);
        assert_eq!(1, queue.size());
        queue.add(3);
        assert_eq!(true, queue.is_full());
        queue.add(4);
        assert_eq!(2, queue.size());
        queue.clear();
        assert_eq!(0, queue.size());
    }

    #[test]
    fn iterator_works() {
        let mut queue = FixedQueue::<i32>::new(3);
        queue.add(2);
        queue.add(3);
        queue.add(4);
        let val=queue.clone().fold(0,|sum,x| sum+x);
        assert_eq!(val, 9);
        println!("{:?}",queue);
    }
}
