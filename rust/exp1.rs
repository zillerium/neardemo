fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    // Your Code here... Happy Coding!
    let mut tot: u64 = 0;
      let mut x5 = a * b * c;
      tot = getmax(x5, tot);
      x5 = a + b + c;
      tot = getmax(x5, tot);
      x5 = a + (b * c);
      tot = getmax(x5, tot);
      x5 = a * (b + c);
      tot = getmax(x5, tot);
      x5 = (a + b) * c;
      tot = getmax(x5, tot);
      x5 = (a * b) + c;
      tot = getmax(x5, tot);
      tot
}

fn getmax (x: u64, y: u64) -> u64 {
    if x>y { return x} else { return y};
}
