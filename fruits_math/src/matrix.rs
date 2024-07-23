use crate::num::Number;

pub trait Matrix<T: Number> {
    const IDENTITY: Self;
}