use std::collections::BTreeMap;
fn html_special_chars(html: &str) -> String {
   // unimplemented!();
    let h = vec![ "&lt;", "&gt;", "&quot;", "&amp;"];
    let v = [('<', 0), ('>', 1), ('\\', 2), ('&', 3) ];
    let n = BTreeMap<char, i32>::from_iter(v.iter().cloned());
    let s = String::new();
    let mut s1 = String::new();
    s1.push_str(html);
    for c in s1.chars() {  
        if c == '<' {
            s.push_str("&lt".to_string())
        } else 
        match n.get(&c) {
            Some(c1) => s.push_str(h[c1 as i32]),
            None => s.push_str(c.to_string())
        }
    }
    return s;
}
