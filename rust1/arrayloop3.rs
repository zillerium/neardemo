extern crate itertools;
use itertools::Itertools;

fn row_weights(array: Vec<u32>) -> (u32, u32) {
    (
        array.iter().step(2).sum(),
        array.iter().skip(1).step(2).sum()
    )
}
