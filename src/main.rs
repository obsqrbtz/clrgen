use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut colors: Vec<String> = vec![String::new(); 18];
    let mut count = 1;
    let file_name = &format!("{}/src/clrgen/example_data/colors", get_home_dir());
    if let Ok(lines) = read_lines(file_name) {
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
    let conf_kitty = generate_kitty_colors(&colors);
    let conf_alacritty = generate_alacritty_colors(&colors);
    write_conf(format!("{}/.config/kitty/gen.conf", get_home_dir()), conf_kitty);
    write_conf(format!("{}/.config/alacritty/gen.yml", get_home_dir()), conf_alacritty);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_conf(file_name: String, conf: Vec<String>){
    fs::write(file_name, conf.join("\n")).expect("");
}

fn get_home_dir() -> String {
    let homedir = env::var("HOME").expect("$HOME is not set");
    homedir
}

fn generate_kitty_colors(colors: &Vec<String>) -> Vec<String> {
    let mut config: Vec<String> = vec![String::new(); 44];
    let mut count = 0;
    if let Ok(lines) 
        = read_lines(format!("{}/src/clrgen/templates/kitty.conf", get_home_dir())) {
        for line in lines {
            if let Ok(conf_line) = line {
                if !conf_line.starts_with("#") && conf_line.len() != 0{
                    let conf_line_split : Vec<_> = conf_line.split_whitespace().map(str::to_string).collect();
                    let option = &conf_line_split[0];
                    let num_str = &conf_line_split[1].replace("$", "");
                    let num_int = num_str.parse::<u8>()
                        .expect("Can not parse 'kitty.conf' template");
                    let value = &colors[num_int as usize].to_string();
                    config[count] = format!("{} {}", option, value);
                }else{
                    config[count] = conf_line;
                }
                count +=1;
            }
        }
    }
    config
}

fn generate_alacritty_colors(colors: &Vec<String>) -> Vec<String> {
    let mut config: Vec<String> = vec![String::new(); 44];
    let mut count = 0;
    if let Ok(lines) 
        = read_lines(format!("{}/src/clrgen/templates/alacritty.yml", get_home_dir())) {
        for line in lines {
            if let Ok(conf_line) = line {
                if !conf_line.starts_with("#") && !conf_line.ends_with(":") && conf_line.len() != 0{
                    let conf_line_split : Vec<_> = conf_line.split_whitespace().map(str::to_string).collect();
                    let option = &conf_line_split[0];
                    let num_str = &conf_line_split[1].replace("$", "");
                    let num_int = num_str.parse::<u8>()
                        .expect("Can not parse 'alacritty.conf' template");
                    let value = &colors[num_int as usize].to_string().replace("#", "");
                    config[count] = format!("    {} '0x{}'", option, value);
                }else{
                    config[count] = conf_line;
                }
                count +=1;
            }
        }
    }
    config
}