#![feature(external_doc)]

pub mod traits;
pub mod helpers;
mod ds;
pub mod math;

pub use ds::fixed_queue;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
