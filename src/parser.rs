

pub fn tokenize(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut has_content = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
         match c {
            '\\' if !in_single_quote && !in_double_quote=>{
                if let Some(next_c) = chars.next(){
                    current.push(next_c);
                    has_content = true;
                }
            }
            '\\' if in_double_quote=>{
                if let Some(&next_c) = chars.peek(){
                    if next_c == '\\' || next_c == '$' || next_c == '"' || next_c == '\n'{
                        current.push(chars.next().unwrap()); 
                }else{
                    current.push('\\');
                }
            }else{
                current.push('\\');
            }
            has_content = true;
            }
            '\'' if !in_double_quote =>{
                in_single_quote = !in_single_quote;
                has_content = true;
            }
            '"' if !in_single_quote =>{
                in_double_quote = !in_double_quote;
                has_content = true;
            }
            c if c.is_whitespace() && !in_single_quote && !in_double_quote =>{
                if has_content{
                    args.push(current.clone());
                    current.clear();
                    has_content=false;
                }
            }

            _ =>{
                current.push(c);
                has_content = true;
            }
        }
    }

    if has_content{
        args.push(current);
    }

    args
}
