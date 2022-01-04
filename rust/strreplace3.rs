fn html_special_chars(html: &str) -> String {
    html.chars().fold(String::new(), |mut s, x| {
       match x {
      '<' => s.push_str("&lt;"),
      '>' => s.push_str("&gt;"),
      '"' => s.push_str("&quot;"),
      '&' => s.push_str("&amp;"),
      _ => s.push(x), 
      };
      s
    })
}
