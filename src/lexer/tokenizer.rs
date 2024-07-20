#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Macro(String),
    StringLit(String),
    Delimeter(String),
    Comment(String),
    NewLine(),
    Whitespace(),
}

const RUST_KEYWORDS: [&str; 10] = [
    "fn", "let", "mut", "const", "static", "pub", "mod", "in", "for", "while",
];

fn keyword_or_identifier(identifier: String, tokens: &mut Vec<Token>) {
    if RUST_KEYWORDS.contains(&identifier.as_str()) {
        tokens.push(Token::Keyword(identifier.clone()));
        return;
    }
    if identifier.ends_with('!') {
        tokens.push(Token::Macro(identifier.clone()));
        return;
    }
    tokens.push(Token::Identifier(identifier.clone()))
}

pub fn tokenize(content: &Vec<String>) -> Vec<Token> {
    let mut tokens = vec![];

    for line in content {
        let mut index = 0;
        let mut identifier = String::new();
        let chars: Vec<char> = line.chars().collect();
        let content_length = line.len();

        while index < content_length {
            match chars[index] {
                '/' => {
                    if chars[index + 1] != '/' {
                        continue;
                    }
                    let is_multiline_comment = chars[index + 1] == '*';
                    let mut comment = String::new();
                    while line.len() > index {
                        comment.push(chars[index]);
                        index += 1;
                    }
                    if is_multiline_comment {
                        // TODO: Add support for multiline comments
                    }
                    tokens.push(Token::Comment(comment.clone()));
                    index += 1;
                }
                '"' => {
                    let mut string_lit = String::new();
                    index += 1;
                    string_lit.push('"');
                    while chars[index] != '"' {
                        string_lit.push(chars[index]);
                        index += 1;
                    }
                    string_lit.push('"');
                    tokens.push(Token::StringLit(string_lit.clone()));
                    index += 1;
                }
                ' ' => {
                    if identifier.is_empty() {
                        tokens.push(Token::Whitespace());
                        index += 1;
                        continue;
                    }
                    keyword_or_identifier(identifier.clone(), &mut tokens);
                    identifier.clear();
                }
                '(' | ')' | '{' | '}' | ';' | '[' | ']' => {
                    if !identifier.is_empty() {
                        keyword_or_identifier(identifier.clone(), &mut tokens);
                        identifier.clear();
                        continue;
                    }
                    tokens.push(Token::Delimeter(chars[index].to_string()));
                    index += 1;
                    let t = tokens
                        .clone()
                        .into_iter()
                        .filter(|token| *token != Token::Whitespace())
                        .collect::<Vec<Token>>();

                    if chars[index - 1] == '('
                        && t.get(t.len() - 3) == Some(&Token::Keyword(String::from("fn")))
                    {
                        let mut function_arguments = String::new();
                        while chars[index] != ')' {
                            function_arguments.push(chars[index]);
                            index += 1;
                        }
                        let function_arguments: Vec<&str> = function_arguments.split(',').collect();
                        for (i, argument) in function_arguments.iter().enumerate() {
                            tokens.push(Token::Keyword(argument.trim().to_string()));
                            if i < function_arguments.len() - 1 {
                                tokens.push(Token::Delimeter(String::from(", ")));
                            }
                        }
                    }
                }
                _ => {
                    identifier.push(chars[index]);
                    index += 1;
                }
            }
        }

        tokens.push(Token::NewLine());
    }

    tokens
}
