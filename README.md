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

## How to use the existing backend
A version of the backend is deployed to `http://32k.eu:8000/api`. It is not guaranteed to be online. See the "Bruno" REST client templates under `bruno`. Auth: You will need to provide a Bearer Token with each request to authenticate.

## How to run the backend
 - Install the rust toolchain via "rustup". [How to install](https://www.rust-lang.org/tools/install)
 - Go to the `backend-rs` dir and create a `.env` file with the following content:
```
DATABASE_URL=sqlite://sqlite.db
USER_TOKEN=SwexCamp2024!
ADMIN_TOKEN=SwexCamp2024Admin!
```
 - Install the sqlite CLI: `sudo apt install sqlite3`
 - Create an empty DB: `sqlite3 sqlite.db "VACUUM;"`
 - Run the migrations: `./run_migrations.sh`
 - Build and run the backend: `cargo run -r`

## How to use the backend

### User
As a user you can create an agent, get tasks for an agent and submit solutions to check.

### Create an agent
POST `/api/agent`, headers:
```
Authorization: Bearer <USER_TOKEN>
```

Body:
```json
{
  "name": "Simple cloudless agent 1.0",
  "usedModelsAndApis": [ "codestral:22b-v0.1-q6_K", "self-hosted", "ollama" ]
}
```

This will return an ID and a token. You will have to remember both:
```json
{
  "id": "019000f4-b747-733e-a48f-23b026250ba7",
  "token": "019000f4-b747-7cc4-a52e-f6f76a498548"
}
```

### Get a list of tasks to solve
GET `api/agent/019000f4-b747-733e-a48f-23b026250ba7/task?token=019000f4-b747-7cc4-a52e-f6f76a498548`,
headers:
```
Authorization: Bearer <USER_TOKEN>
```
You will get a list of tasks (completed and not yet completed):
```json
[
  {
    "id": "018ff1a3-7f7f-7796-a7b1-82cc4f89cbf5",
    "name": "Warm-up task 1",
    "completed": false,
    "time_in_ms": 10000
  },
  {
    "id": "018ff1bd-4803-74da-bf82-3448223365cb",
    "name": "Warm-up task 2",
    "completed": false,
    "time_in_ms": null
  },
  //...
]
```

### Get a specific task
GET `api/agent/018ff1bb-644b-7506-87ce-8df4aff14ac2/task/018ff1bd-4803-74da-bf82-3448223365cb?token=018ff1bb-644b-72b2-a67e-d4f125227f1f`,
headers:
```
Authorization: Bearer <USER_TOKEN>
```
You will get a task with a description. Calling this for the first time will start the timer.
```json
{
  "name": "Warm-up task 2",
  "taskType": {
    "SimpleTask": {
      "description": "What is 100 - 2?"
    }
  }
}
```

### Submit a solution
POST `api/agent/018ff1bb-644b-7506-87ce-8df4aff14ac2/task/018ff1bd-4803-74da-bf82-3448223365cb/check?token=018ff1bb-644b-72b2-a67e-d4f125227f1f`,
headers:
```
Authorization: Bearer <USER_TOKEN>
```
Body:
```json
{
  "solution" : "2"
}
```
You will receive a response indicating if the solution was correct:
```json
{
  "correct": true
}
```
If the solution is correct the timer is stopped and the `time_in_ms` (see previous request) gets updated.
