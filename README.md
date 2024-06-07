# Advent of AI: Meta hackathon

## What
Explore the capabilities and limitations of current LLM-based software development agents by trying to implement one ourselves.

Link to advent of code: [Advent of Code](https://adventofcode.com/) is a yearly contest featuring 25 coding puzzles to solve in any programming language. The goal of our hackathon is to solve as many 2023 puzzles as possible.

### The Twist:
As participants we cannot directly write code to solve the puzzles. Instead, we use agentic AI tools to do that for us.

 - Only prompts, glue code, API integration and agent logic (e.g. pass compile errors back to the agent) can be written.
 - We utilize APIs from providers like OpenAI, Groq, Anthropic, Google, etc.
 - Prompts must not be puzzle specific.


### Hard Mode:

The AI agent must be re-run from puzzle 1 after each modification without retaining memory of previous runs. It starts from scratch every time.

### Why Advent of Code?

A single puzzle is usually self-contained, not super hard and the solution (which should fit into the context length of a conventional model) can be verified easily. The puzzles are given in text form and solving them involves no multi-modality.

## How to run the backend
 - Install the rust toolchain via "rustup". [How to install](https://www.rust-lang.org/tools/install)
 - Go to the `backend-rs` dir and create a `.env` file with the following content:
```
DATABASE_URL=sqlite://sqlite.db
BEARER_TOKEN=SwexCamp2024!
```
 - run `./run_migrations.sh`
 - run `cargo run -r`
