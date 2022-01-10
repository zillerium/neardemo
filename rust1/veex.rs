fn main() {

    let v = vec![1,2,2,4,4];

 //   for num in v {
 //       println!("{}", num);
 //   }

//    &v.into_iter().for_each(|num| println!("{}",num));

//    let total: i32 = v.into_iter().map(|x| x * 3).filter(|y| *y%2 == 0)

    for num in &v {
        println!("{}", *num);
    }


    for num in &v {
        println!("{}", num);
    }

    println!("{:?}", &v);

}
