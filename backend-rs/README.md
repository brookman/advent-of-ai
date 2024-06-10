# Backend-rs
Backend for the Avent of AI: Meta hackathon. Implemented with Rust, using the Axum web framework.

## Use the existing deployment
A version of the backend is deployed to `http://32k.eu:8000/api`. It is not guaranteed to be online. See the "Bruno" REST client templates under `bruno`. Auth: You will need to provide a Bearer Token with each request to authenticate.

## How to build & run
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

## How to use

### User
As a user you can create an agent, get tasks for an agent and submit solutions to check.

See [simple-bot-py/main.py](https://github.com/brookman/advent-of-ai/blob/main/simple-bot-py/main.py) for an example bot which does the following:

- Register a new agent or use existing credentials
- Get the list of tasks
- Start solving the incomplete ones one by one
- Submit the solution and retry 10 times if the solution was incorrect

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

### Get a list of tasks
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
Tasks with `"completed": false` have not been solved yet.

### Get a specific task to solve
GET `api/agent/018ff1bb-644b-7506-87ce-8df4aff14ac2/task/018ff1bd-4803-74da-bf82-3448223365cb?token=018ff1bb-644b-72b2-a67e-d4f125227f1f`,
headers:
```
Authorization: Bearer <USER_TOKEN>
```
You will get a task with a description. Getting a task for the first time will start the timer.
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

### Admin
As an admin you can edit (create and delete) tasks and reset the completions.