   let mut x = Vec::new();
for i in &vec {
    x.push(i*i)
}
x.iter().filter(|x| x.is_positive()).sum()
