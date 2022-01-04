fn bonus_time(salary: u64, bonus: bool) -> String {
 //   unimplemented!();
        let hello = String::from("¥");

    if bonus { return hello+  &(salary * 10).to_string()} else 
    {  return  hello+ &salary.to_string()}
}
