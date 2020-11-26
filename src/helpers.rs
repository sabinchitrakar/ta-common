pub fn trend(prev: f64, current: f64) -> [f64; 2] {
    let mut up = 0.0;
    let mut down = 0.0;
    if prev > current {
        down = prev - current;
    }
    if current > prev {
        up = current - prev;
    }

    [up, down]
}


#[cfg(test)]
mod tests {
    use crate::helpers::trend;

    #[test]
    fn trend_works() {
        assert_eq!([50.0, 0.0], trend(100.0, 150.0));
        assert_eq!([0.0, 50.0], trend(150.0, 100.0));
        assert_eq!([0.0, 0.0], trend(100.0, 100.0));
    }
}
