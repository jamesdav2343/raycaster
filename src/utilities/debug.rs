pub fn pretty_print_vec<T>(vec: &Vec<T>, row_length: usize, custom_gap: Option<usize>)
where
    T: TryInto<u64> + ToString + Copy + Ord,
{
    let format_max_gap: usize = custom_gap.unwrap_or_else(|| {
        vec.iter()
            .map(|&val| val.try_into().unwrap_or(0))
            .max()
            .unwrap_or(0)
            .to_string()
            .len()
    });

    let grid = &mut String::from("");

    for (i, item) in vec.iter().enumerate() {
        if (i % row_length == 0) && (i != 0) {
            grid.push_str("\n");
        }

        append_formatted_grid_item(item, format_max_gap, grid);
    }

    println!("{}", grid);
}

fn append_formatted_grid_item<T>(&item: &T, max_gap: usize, grid: &mut String)
where
    T: TryInto<u64> + ToString + Copy + Ord,
{
    let cast_item = item.try_into().unwrap_or(0);

    let chars: Vec<char> = cast_item.to_string().chars().collect();

    let num_digits: u8 = chars.len() as u8;

    for _i in 0..(max_gap as u8 - num_digits) {
        grid.push_str(" ");
    }

    grid.push_str(&format!("{}, ", &cast_item.to_string()));
}
