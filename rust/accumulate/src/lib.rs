/// What should the type of _function be?
pub fn map<T, F, O>(input: Vec<T>, mut function: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    let mut result = vec![];

    for n in input {
        result.push(function(n))
    }

    result
}
