
pub fn parser_input(input: &str)-> Vec<String>{
    let mut args  = Vec::new();
    let mut current_args = String::new();
    let mut is_quote = false;

    for c in input.trim().chars(){
        match c{
         '"' =>{
                is_quote = !is_quote;
        }

        ' ' if !is_quote =>{
            if !current_args.is_empty(){
                args.push(current_args.clone());
                current_args.clear();
            }
        }

        _ =>{
            current_args.push(c);
        }
        }
    }

    if !current_args.is_empty(){
        args.push(current_args);
    }

    return args;
}