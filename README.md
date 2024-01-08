# ✂️ parsnip

This is a research project that explores using a BNF grammar to incrementally constrain a stateful string as it is being generated.

In human terms, this is like trying to type a sentence, but only being able to type the next letter if it is a valid next letter in the sentence. 

why?

I'm interested in constraining the output of large language models to fixed grammars. This is a first step in that direction.

notes:

this repo includes tests that will randomly choose from the allowed next characters, so the output will be different each time but constrained by the grammar.

this repo is not optimized for performance and reapplies the grammar rules each time a new character is generated. This is not necessary and could be optimized.

luckily it's not terribly slow and is currently running at about 552.2K tokens/sec on my machine; adding roughtly 100ms per generation step with 51200 tokens.

try a random generation example with the following command:

```bash
cargo run --release --example dna
```