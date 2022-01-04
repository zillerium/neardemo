fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    //let   input1 = input;
    //let _n: i32 = input1.into_iter().filter( |c| c < &0).collect::<Vec<_>>().len() as i32;
    //let _p:i32  = input1.into_iter().filter( |c| c > &0).collect::<Vec<_>>().len() as i32;
    //return vec![_p, _n]
    let mut n = 0;
    let mut  p = 0;
    for i in input {
        if i>0 {p+=1;} else if i<0 {n+=i;}
    }
    if p == 0 && n == 0 {
    return vec![];
    } else {
        
    }  return vec![p,n];
}
