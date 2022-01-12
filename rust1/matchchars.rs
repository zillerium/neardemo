 let indices1 = letters.char_indices().filter_map(|(i, c)| match c == 'a' {
        true => Some(i),
        false => None,
    }).collect::<Vec<_>>();
