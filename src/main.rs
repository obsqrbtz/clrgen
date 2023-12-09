mod gen;
mod common;

fn main() {
    let mut colors: Vec<String> = vec![String::new(); 18];
    let mut count = 1;
    let file_name = &format!("{}/src/clrgen/example_data/colors", common::get_home_dir());
    if let Ok(lines) = common::read_lines(file_name) {
        for line in lines {
            if let Ok(conf_line) = line {
                if !conf_line.starts_with("#") && conf_line.len() != 0{
                    let conf_line_split : Vec<_> = conf_line.split_whitespace().map(str::to_string).collect();
                    let num_str = &conf_line_split[0];
                    let num_int = num_str.parse::<u8>()
                    .expect(&format!("Can not parse color number at line {} in '{}'", count, file_name));
                    let color = &conf_line_split[1];
                    colors[num_int as usize] = color.to_string();
                }
                count +=1;
            }
        }
    }
    let conf_kitty = gen::generate_kitty_colors(&colors);
    let conf_alacritty = gen::generate_alacritty_colors(&colors);
    common::write_conf(format!("{}/.config/kitty/gen.conf", common::get_home_dir()), conf_kitty);
    common::write_conf(format!("{}/.config/alacritty/gen.yml", common::get_home_dir()), conf_alacritty);

}