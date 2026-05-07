pub fn pretty_print_vec(vec: &Vec<u8>, row_length: usize) {
    let format_max_gap: usize = vec.iter().max().unwrap().to_string().len();
    let grid = &mut String::from("");

    for (i, item) in vec.iter().enumerate() {
        if (i % row_length == 0) && (i != 0) {
            grid.push_str("\n");
        }

        append_formatted_grid_item(item, format_max_gap, grid);
    }

    println!("{}", grid);
}

fn append_formatted_grid_item(&num: &u8, max_gap: usize, grid: &mut String) {
    let chars: Vec<char> = num.to_string().chars().collect();

    let num_digits: u8 = chars.len() as u8;

    for _i in 0..(max_gap as u8 - num_digits) {
        grid.push_str(" ");
    }

    grid.push_str(&format!("{}, ", &num.to_string()));
}
