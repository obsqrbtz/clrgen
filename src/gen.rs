use crate::common;

pub fn generate_kitty_colors(colors: &Vec<String>) -> Vec<String> {
    let mut config: Vec<String> = vec![String::new(); 44];
    let mut count = 0;
    if let Ok(lines) 
        = common::read_lines(format!("{}/.clrgen/templates/kitty.conf", common::get_home_dir())) {
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

pub fn generate_alacritty_colors(colors: &Vec<String>) -> Vec<String> {
    let mut config: Vec<String> = vec![String::new(); 44];
    let mut count = 0;
    if let Ok(lines) 
        = common::read_lines(format!("{}/.clrgen/templates/alacritty.yml", common::get_home_dir())) {
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