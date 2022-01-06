use std::mem;

fn use_slice(slice: &mut [i32]) {
slice[1]=67;
println!("{:?} {}", slice, slice.len());
}

fn slices() {
let mut data = [1,2,3,4,5];
use_slice(&mut data[1..5]);
}

fn main() {
slices();
    let mut a: [i32;5] = [1,2,3,4,5];
    let mut b = [1i32;5];
    println!("{} {:?}" , a.len(), a);
    println!("{} {:?} {}" , b.len(), b, mem::size_of_val(&b));

    let mtx:[[f32;2];2]=
        [
            [1.0, 2.0], 
            [3.0, 4.0]
        ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i==j { println!("link {}", mtx[i][j]) }
        }


    }
}
