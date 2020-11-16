# ta-common
Common Traits For Technical Indicators

### Fixed Queue
```
use ta_common::fixed_queue::FixedQueue;
let mut queue=FixedQueue::<i32>::new(2);
queue.add(2);
assert_eq!(1,queue.size());
queue.add(3);
assert_eq!(true,queue.is_full());
queue.add(4);
assert_eq!(2,queue.size());
queue.clear();
assert_eq!(0,queue.size());
```
