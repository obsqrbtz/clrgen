mod gen;
mod common;

fn main() {
    let mut colors: Vec<String> = vec![String::new(); 18];
    let mut count = 1;
    let file_name = &format!("{}/.clrgen/colors/colors", common::get_home_dir());
    if let Ok(lines) = common::read_lines(file_name) {
        for line in lines {
            if let Ok(conf_line) = line {
                if !conf_line.starts_with("#") && conf_line.len() != 0{
                    let conf_line_split : Vec<_> = conf_line.split_whitespace().map(str::to_string).collect();
                    let num_str = &conf_line_split[0];
                    let num_int = num_str.parse::<u8>()
                    .expect(&format!("Can not parse color number at line {} in '{}'", count, file_name));
                    let color = &conf_line_split[1];
                    colors[num_int as usize] = color.to_string().replace("#", "");
                }
                count +=1;
            }
        }
    }
    let conf_kitty = gen::generate_hex(&colors, "kitty.conf".to_string());
    let conf_i3 = gen::generate_hex(&colors, "i3config".to_string());
    let conf_alacritty = gen::generate_hex(&colors, "alacritty.yml".to_string());
    let conf_eww = gen::generate_hex(&colors, "eww.scss".to_string());
    let conf_sway = gen::generate_hex(&colors, "sway".to_string());
    let conf_waybar = gen::generate_hex(&colors, "waybar.css".to_string());
    common::write_conf(format!("{}/.clrgen/kitty_clrgen.conf", common::get_home_dir()), conf_kitty);
    common::write_conf(format!("{}/.clrgen/alacritty_clrgen.yml", common::get_home_dir()), conf_alacritty);
    common::write_conf(format!("{}/.clrgen/i3_clrgen", common::get_home_dir()), conf_i3);
    common::write_conf(format!("{}/.clrgen/eww.scss", common::get_home_dir()), conf_eww);
    common::write_conf(format!("{}/.clrgen/sway", common::get_home_dir()), conf_sway);
    common::write_conf(format!("{}/.clrgen/waybar.css", common::get_home_dir()), conf_waybar);

}