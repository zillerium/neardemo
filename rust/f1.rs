
fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (1..).map(|x| x*n).take_while(|x| x<= &limit).collect()
}
