fn html_special_chars(html: &str) -> String {
    let mut s = String::new();
    let mut s1 = String::new();
    s1.push_str(html);
    println!("{:?}", s1);
    for c in s1.chars() {
          println!("{:?}", s);

        if c == '<' {
            s.push_str(&"&lt;".to_string())
        } else if c == '>' {
            s.push_str(&"&gt;".to_string())
        } else if c == '&' {
            s.push_str(&"&amp;".to_string())
        } else if c == '\"' {
            s.push_str(&"&quot;".to_string())
        } else {
            s.push_str(&c.to_string())
        }

    }
    return s;
}
