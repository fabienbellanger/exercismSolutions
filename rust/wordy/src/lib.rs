#[derive(Debug)]
enum CustomError {
    InvalidOperation,
    DivisionByZero,
}

#[derive(Debug, Clone)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
}

impl TryFrom<&str> for Operation {
    type Error = CustomError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "plus" => Ok(Self::Addition),
            "minus" => Ok(Self::Subtraction),
            "multiplied_by" => Ok(Self::Multiplication),
            "divided_by" => Ok(Self::Division),
            "raised" => Ok(Self::Power),
            _ => Err(CustomError::InvalidOperation),
        }
    }
}

impl Operation {
    fn calculate(&self, x: i32, y: i32) -> Result<i32, CustomError> {
        match self {
            Self::Addition => Ok(x + y),
            Self::Subtraction => Ok(x - y),
            Self::Multiplication => Ok(x * y),
            Self::Division => {
                if y == 0 {
                    return Err(CustomError::DivisionByZero);
                }
                Ok(x / y)
            }
            Self::Power => Ok(x.pow(y as u32)),
        }
    }
}

fn parse_command(command: &str) -> Option<String> {
    let command = command.strip_prefix("What is ")?;
    let command = command.strip_suffix('?')?;
    let command = &command.replace("multiplied by", "multiplied_by");
    let command = &command.replace("divided by", "divided_by");
    let command = &command.replace("raised to the", "raised");
    let command = &command.replace("st power", "");
    let command = &command.replace("th power", "");
    let command = &command.replace("nd power", "");
    let command = &command.replace("rd power", "");
    Some(command.to_string())
}

pub fn answer(command: &str) -> Option<i32> {
    let command = parse_command(command)?;
    let items = command.split_whitespace().collect::<Vec<_>>();

    if items.is_empty() {
        return None;
    }

    let mut has_error = false;
    let mut current_operation: Option<Operation> = None;
    let mut current_number = 0;
    items.iter().enumerate().for_each(|(i, &item)| {
        if has_error {
            return;
        }

        if i % 2 == 0 {
            // Must be a number
            if let Ok(number) = item.parse::<i32>() {
                if let Some(operation) = current_operation.clone() {
                    if let Ok(r) = operation.calculate(current_number, number) {
                        current_operation = None;
                        current_number = r;
                    } else {
                        has_error = true;
                    }
                } else {
                    current_number = number;
                }
            } else {
                has_error = true;
            }
        } else {
            // Must be an operation
            if let Ok(operation) = Operation::try_from(item) {
                current_operation = Some(operation);
            } else {
                has_error = true;
            }
        }
    });

    if has_error || current_operation.is_some() {
        None
    } else {
        Some(current_number)
    }
}
