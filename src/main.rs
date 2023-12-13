mod gen;
mod common;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the color theme.
    #[arg(short, long)]
    theme: std::path::PathBuf,

    /// Target app. Example: kitty, i3, all.
    /// For full app list visit https://github.com/obsqrbtz/clrgen.
    #[arg(short, long, 
        default_value_t = format!("all"))]
    appname: String,

    /// Path to config file.
    #[arg(short, long, 
        default_value = format!("{}/.config/clrgen/clrgen.conf", common::get_home_dir()))]
    config: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    let mut colors: Vec<String> = vec![String::new(); 18];
    let mut count = 1;
    let theme_path = args.theme;
    if let Ok(lines) = common::read_lines(&theme_path) {
        for line in lines {
            if let Ok(conf_line) = line {
                if !conf_line.starts_with("#") && conf_line.len() != 0{
                    let conf_line_split : Vec<_> = conf_line.split_whitespace().map(str::to_string).collect();
                    let num_str = &conf_line_split[0];
                    let num_int = num_str.parse::<u8>()
                    .expect(&format!("Can not parse color number at line {} in '{}'", 
                        count, &theme_path.display()));
                    let color = &conf_line_split[1];
                    colors[num_int as usize] = color.to_string().replace("#", "");
                }
                count +=1;
            }
        }
    }
    if args.appname == "all"{

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
    }else{
        let appname = args.appname.as_str();
        match appname{ 
            "kitty" => {
                let conf_kitty = gen::generate_hex(&colors, "kitty.conf".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_kitty.conf", common::get_home_dir()), conf_kitty);
            }, 
            "alacritty" => {
                let conf_alacritty = gen::generate_hex(&colors, "alacritty.yml".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_alacritty.yml", common::get_home_dir()), conf_alacritty);
            }, 
            "i3" => {
                let conf_i3 = gen::generate_hex(&colors, "i3config".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_i3", common::get_home_dir()), conf_i3);
            },
            "eww"=> {
                let conf_eww = gen::generate_hex(&colors, "eww.scss".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_eww.scss", common::get_home_dir()), conf_eww);
            },
            "sway" => {
                let conf_sway = gen::generate_hex(&colors, "sway".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_sway", common::get_home_dir()), conf_sway);
            },
            "waybar" => {
                let conf_waybar = gen::generate_hex(&colors, "waybar.css".to_string());
                common::write_conf(format!("{}/.clrgen/clrgen_waybar.css", common::get_home_dir()), conf_waybar);
            },
            _ => println!("No template for the app {}", args.appname),
        }
    }

}