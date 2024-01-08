use parsnip::StatefulString;
use rand::seq::SliceRandom; // Import the SliceRandom trait for random selection
use rand::thread_rng;

fn main() {
    let bnf_grammar = "<sum> ::= <sum> <add> <product>
    <sum> ::= <product>
    <product> ::= <product> <mult> <factor>
    <product> ::= <factor>
    <add> ::= '+' | '-'
    <mult> ::= '*' | '/'
    <factor> ::= '(' <sum> ')'
    <factor> ::= <number>
    <number> ::= <digit> <number>
    <number> ::= <digit>
    <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'";

    let grammar = bnf_grammar.parse().unwrap();

    let token_candidates = vec![
        'Z', 'X', 'Y', 'A', 'C', 'G', 'T', '(', ')', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
        'j', '}', ',', ':', '"', ' ', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '+', '-',
    ];

    let mut stateful_string = StatefulString::new(grammar);
    let mut rng = thread_rng();

    stateful_string.append_token('(');
    stateful_string.append_token('1');
    stateful_string.append_token('+');
    
    let mut count = 0;
    // Use a loop to keep appending tokens
    while !stateful_string.is_complete_and_valid().0 || count < 30 {
        let next_tokens = stateful_string.get_next_token_options(&token_candidates);
        if !next_tokens.is_empty() {
            if let Some(&token) = next_tokens.choose(&mut rng) {
                count += 1;
                stateful_string.append_token(token);
            }
        }
    }

    println!("Generated string: {}", stateful_string.current_text);
}
