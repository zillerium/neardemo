fn row_weights(array: Vec<u32>) -> (u32, u32) {
    // your code here
    let mut e:u32 = 0;
    let mut o:u32 = 0;
    let mut iter = array.iter();
    for i in 0..array.len() {
        if i % 2 == 0 {
             e += array[i];
        } else {
            o  += array[i];
        }
    }
    (e, o)
    
}
