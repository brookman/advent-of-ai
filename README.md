# Advent of AI: Meta Hackathon

Explore the capabilities and limitations of current LLM-based software development agents by implementing one yourself.

## What is an AI Agent?

For this hackathon, an *AI agent* is defined as a piece of software that autonomously solves a given task by repeatedly calling an LLM. The agent receives an unsolved task from the backend, writes code, executes the code, and submits a solution.

## The Challenge

We will be participating in the [Advent of Code](https://adventofcode.com/), a yearly contest featuring 25 (times 2) coding puzzles to solve in any programming language. The goal of our hackathon is to solve as many of the 2023 puzzles as possible.

### The Twist

Participants are not allowed to directly write code to solve the puzzles. Instead, we must use agentic AI tools to achieve this.

- Only prompts, glue code, API integration, and agent logic (e.g., passing compile errors back to the agent) can be written.
- Utilize APIs from providers like OpenAI, Groq, Anthropic, Google, etc.
- Prompts must not be puzzle-specific.

### Hard Mode

The AI agent must be re-run from puzzle 1 after each modification without retaining memory of previous runs. It starts from scratch every time.

## Why Advent of Code?

- Each puzzle is usually self-contained and not overly complex.
- The solution should fit within the context length of a conventional model and can be easily verified.
- The puzzles are given in text form, requiring no multi-modality.

## How to Use the Backend

A version of the Rust backend is deployed at `http://32k.eu:8000/api`. Availability is not guaranteed. See the "Bruno" REST client templates under `bruno`. 

### Authentication

You will need to provide a Bearer Token with each request to authenticate (ask Beni).

For detailed information on how to build, run, and use the backend, see [backend-rs](backend-rs).

## Example Agent

A simple agent implementation can be found under [simple-bot-py](simple-bot-py).