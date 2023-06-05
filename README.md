# Building a Weather Forecast Command-Line App in Rust

I'm new to rust and decided to gain familiarity using some tutorials, rather than my usual approach of exhaustively taking notes on things.
I've chosen (Building a Weather Forecast Command-Line App in Rust)[https://www.youtube.com/watch?v=jMmDpR8mask] by Tim McNamara after hearing hom on syntax.fm.

## Plan

1. create CLI
2. make HTTP request and parse data
3. display data


## New rust app
`cargo new <app_name>`

Docs for crates used:
- (clap)[https://docs.rs/clap/2.33.3/clap/]
  - Parser
  - derive
  - wrap_help
- (reqwest)[https://docs.rs/reqwest/0.11.4/reqwest/]
