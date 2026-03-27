pub fn tokenize(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut has_content = false;

    for c in input.chars() {
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
            } else {
                current.push(c);
                has_content = true;
            }
        } else {
            if c == '\'' {
                in_single_quote = true;
                has_content = true;
            } else if c.is_whitespace() {
                if has_content {
                    result.push(current.clone());
                    current.clear();
                    has_content = false;
                }
            }else{
                current.push(c);
                has_content= true;
            }
        }
    }

    if has_content{
        result.push(current);
    }

    result
}
