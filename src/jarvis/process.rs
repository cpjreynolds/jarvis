pub struct Process {
    exec: fn(),
    env: Environment,
}

pub struct Environment {
    args: Vec<String>,
    status: i32,
}


