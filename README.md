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

## How to use the backend
A version of the Rust backend is deployed to `http://32k.eu:8000/api`. It is not guaranteed to be online. See the "Bruno" REST client templates under `bruno`. Auth: You will need to provide a Bearer Token with each request to authenticate.
For detailed information on how to build, run and use the backend see [backend-rs](backend-rs).

## Example agent
Find a simple agent implementation under [simple-bot-py](simple-bot-py).