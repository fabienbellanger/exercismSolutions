#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(v) => stack.push(*v),
            _ => match (stack.pop(), stack.pop()) {
                (Some(x), Some(y)) => match input {
                    CalculatorInput::Add => stack.push(y + x),
                    CalculatorInput::Subtract => stack.push(y - x),
                    CalculatorInput::Multiply => stack.push(y * x),
                    CalculatorInput::Divide => stack.push(y / x),
                    _ => return None,
                },
                _ => return None,
            },
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
