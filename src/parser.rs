

pub fn tokenize(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut has_content = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
         match c {
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
