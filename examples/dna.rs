use parsnip::StatefulString;
use rand::seq::SliceRandom; // Import the SliceRandom trait for random selection
use rand::thread_rng;

fn main() {
    let bnf_grammar = "<dna> ::= <base> | <base> <dna>
    <base> ::= 'A' | 'C' | 'G' | 'T'";

    let grammar = bnf_grammar.parse().unwrap();

    let token_candidates = vec![
        'Z', 'X', 'Y', 'A', 'C', 'G', 'T', '(', ')', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
        'j', '}', ',', ':', '"', ' ', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '+', '-',
    ];

    let mut stateful_string = StatefulString::new(grammar);
    let mut rng = thread_rng();

    stateful_string.append_token('A');
    stateful_string.append_token('C');

    let mut count = 0;

    // Use a loop to keep appending tokens
    while !stateful_string.is_complete_and_valid().0 || count < 30 {
        let next_tokens = stateful_string.get_next_token_options(&token_candidates);
        if !next_tokens.is_empty() {
            count += 1;
            if let Some(&token) = next_tokens.choose(&mut rng) {
                stateful_string.append_token(token);
            }
            println!("{}", stateful_string.current_text);
        }
    }

    println!("Generated string: {}", stateful_string.current_text);
}
