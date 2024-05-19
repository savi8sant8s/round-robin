pub fn generate_rounds<T>(mut values: Vec<T>) -> Vec<(T, T)> 
where 
    T:Clone
{
    if values.len() % 2 != 0 {
        return vec![];
    }

    let mut first_half = Vec::<(T, T)>::new();
    let mut second_half = Vec::<(T, T)>::new();

    for n in 0..(values.len() / 2) {
        if n == values.len() / 2 - 1 {
            let first_value = values[values.len() - n - 1].clone();
            let second_value = values[n].clone();

            first_half.push((
                first_value,
                second_value
            ));
        } else {
            first_half.push((
                values[n].clone(),
                values[values.len() - n - 1].clone()
            ));
        }
    }

    values[1..].rotate_right(1);

    let mut results = Vec::<(T, T)>::new();
    for _ in 1..(values.len() - 1) {
        for i in 0..(values.len() / 2) {
            if i == values.len() / 2 - 1 {
                second_half.push((
                    values[values.len() - i - 1].clone(),
                    values[i].clone()
                ));
            } else {
                second_half.push((
                    values[i].clone(),
                    values[values.len() - i - 1].clone()
                ));
            }
        }
    }

    results.extend(first_half);
    results.extend(second_half);

    results
}

