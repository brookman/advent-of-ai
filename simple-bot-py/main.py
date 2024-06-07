import requests


def main():
    print('Starting the bot...')
    
    base_url = 'http://32k.eu:8000/api/'
    auth_token = 'SwexCamp2024!'
    
    headers = {
        'Authorization': 'Bearer ' + auth_token
    }
    
    # get previously stored agent_id and agent_token from the file system
    agent_id = None
    agent_token = None
    try:
        with open('agent_id', 'r') as f:
            agent_id = f.read()
        with open('agent_token', 'r') as f:
            agent_token = f.read()
    except FileNotFoundError:
        pass
    
    
    if agent_id is None or agent_token is None:
        print('Creating a new agent...')
        
        create_agent_request = {
            'name': 'simple-bot.py',
            'usedModelsAndApis': []
        }
        
        response_json = requests.post(base_url + 'agent', json=create_agent_request, headers=headers).json()
        print('Resonse:', response_json)
        
        agent_id = response_json['id']
        agent_token = response_json['token']
        
        # store the agent_id and agent_token on the file system
        with open('agent_id', 'w') as f:
            f.write(agent_id)
        with open('agent_token', 'w') as f:
            f.write(agent_token)
    else:
        print('Agent already exists')
    
    tasks_json = requests.get(base_url + 'agent/' + agent_id + '/task?token=' + agent_token, headers=headers).json()
    
    # iterate over all tasks
    for task in tasks_json:
        task_name = task['name']
        completed = task['completed']
        
        if not completed:
            for i in range(5):
                solution = solve_task(base_url, agent_id, task, agent_token, headers)
                correct = check_solution(base_url, agent_id, task, solution, agent_token, headers)
                
                print('Solution was correct?', correct)
                
                if correct:
                    break
            
        else:
            print('Task already completed:', task_name)
    
    tasks_json = requests.get(base_url + 'agent/' + agent_id + '/task?token=' + agent_token, headers=headers).json()
    print('Tasks ', tasks_json)
        
def solve_task(base_url, agent_id, task, agent_token, headers):
    task_id = task['id']
    task_name = task['name']
    print('Trying to solve task:', task_name)
            
    task_json = requests.get(base_url + 'agent/' + agent_id + '/task/' + task_id + '?token=' + agent_token, headers=headers).json()
    
    task_type = task_json['taskType']

    if 'SimpleTask' in task_type:
        description = task_type['SimpleTask']['description']
        return solve_simple_task(description)
    elif 'AdventOfCodePartOne' in task_type:
        description = task_type['AdventOfCodePartOne']['description']
        input_text = task_type['AdventOfCodePartOne']['input']
        return solve_advent_of_code_part1(description, input_text)
    elif 'AdventOfCodePartTwo' in task_type:
        description = task_type['AdventOfCodePartTwo']['description']
        input_text = task_type['AdventOfCodePartTwo']['input']
        return solve_advent_of_code_part2(description, input_text)
    else:
        raise NotImplementedError('Task type not implemented')

def solve_simple_task(description):
    ollama_url = 'http://192.168.3.2:1337/api/generate'
     
    prompt = '''You are a puzzle/task solver bot. You are given a task to solve. Think out step-by-step and put the final solution in backticks at the end of your response.
    For example, if the solution is 42, you should return `42`. The task is: {}'''.format(description)
    
    request = {
        'model': 'codestral',
        'prompt': prompt,
        'stream': False
    }
    
    response = requests.post(ollama_url, json=request).json()['response']
    print('Response:', response)
    
    if response.count('`') >= 2:
        start = response.rfind('`')
        end = response.rfind('`', 0, start)
        solution = response[end + 1:start]
        return solution
    
    return 'no idea'

def solve_advent_of_code_part1(description, input_text):
    prompt = '''You are an expert "Advent of code" challenge solver. You are given a coding puzzle and you generate correct (compilable), efficient and idiomatic Rust code which solves the puzzle for a
given input. Think out step-by-step first and put the code solution of your response in the following template format.

```rust
{}
```

The challenge is:
{}'''
    return solve_advent_of_code(description, input_text, prompt)

def solve_advent_of_code_part2(description, input_text):
    prompt = '''You are an expert "Advent of code" challenge solver. You are given a coding puzzle and you generate correct (compilable), efficient and idiomatic Rust code which solves the puzzle for a
given input. You must only solve the task from "Part Two" of the challenge. Think out step-by-step first and put the code solution of your response in the following template format.

```rust
{}
```

The challenge is:
{}'''
    return solve_advent_of_code(description, input_text, prompt)
    
    
def solve_advent_of_code(description, input_text, prompt):
    ollama_url = 'http://192.168.3.2:1337/api/generate'
    
    template_file = '../solver-rs/src/main_template.rs'
    main_file = '../solver-rs/src/main.rs'
    input_file = '../solver-rs/input.txt'
    
    with open(template_file, 'r') as f:
        template = f.read()
     
    prompt = prompt.format(template, description)
    
    request = {
        'model': 'codestral',
        'prompt': prompt,
        'stream': False
    }
    
    response = requests.post(ollama_url, json=request).json()['response']
    print('Response:', response)  
        
    # find the rustcode inside the backticked rust code block (starting with ```rust and ending with ```)
    start = response.find('```rust')
    end = response.find('```', start + 1)
    rust_code = response[start + 7:end]
    print('Rust code:', rust_code)
    
    with open(main_file, 'w') as f:
        f.write(rust_code)
        
    with open(input_file, 'w') as f:
        f.write(input_text)
    
    # run cargo run in the solver-rs directory and get its output (last line only):
    import subprocess
    process = subprocess.run(['cargo', 'run'], cwd='../solver-rs', capture_output=True, text=True)
    errors = process.stderr.strip()
    print('Errors:', errors)
    solution = process.stdout.strip()
    
    print('Rust output:', solution)
    
    return solution

def check_solution(base_url, agent_id, task, solution, agent_token, headers):
    task_id = task['id']
    
    print('Checking solution:', solution)
    
    check_task_request = {
        'solution': solution
    }
    result = requests.post(base_url + 'agent/' + agent_id + '/task/' + task_id + '/check?token=' + agent_token, json=check_task_request, headers=headers).json()
    
    return result['correct']



if __name__ == "__main__":
    main()
