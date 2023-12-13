mod gen;
mod common;
use clap::Parser;
use std::{fs, process::exit};
use serde_derive::Deserialize;
use toml;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the color theme.
    #[arg(short, long)]
    theme: std::path::PathBuf,

    /// Target app. Example: kitty, i3, use_config.
    /// For full app list visit https://github.com/obsqrbtz/clrgen.
    #[arg(short, long, 
        default_value_t = format!("use_config"))]
    appname: String,

    /// Path to config file.
    #[arg(short, long, 
        default_value = format!("{}/.config/clrgen/clrgen.toml", common::get_home_dir()))]
    config: std::path::PathBuf,
}

#[derive(Deserialize)]
struct AppsData {
    apps: Apps,
}

#[derive(Deserialize)]
struct Apps {
    names: Vec<String>,
    templates: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut colors: Vec<String> = vec![String::new(); 18];
    let mut count = 1;
    let theme_path = args.theme;

    let toml_contents = match fs::read_to_string(&args.config) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", &args.config.display());
            exit(1);
        }
    };
    let toml_data: AppsData = match toml::from_str(&toml_contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", &args.config.display());
            exit(1);
        }
    };
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
    if args.appname == "use_config" {
        for i in 0..toml_data.apps.names.len(){
            let conf = create_conf(&toml_data.apps.names[i], &colors);
            common::write_conf(format!("{}/.clrgen/clrgen_{}", common::get_home_dir(), &toml_data.apps.templates[i]), conf);
        }
    }else
    {
        let appname = args.appname.as_str();
        // TODO: handle missing template
        let app_idx = toml_data.apps.names.iter().position(|r| r == appname).unwrap();
        let conf = create_conf(appname, &colors);
        common::write_conf(format!("{}/.clrgen/clrgen_{}", common::get_home_dir(), toml_data.apps.templates[app_idx]), conf);
    }
}

fn create_conf(appname: &str, colors: &Vec<String>) -> Vec<String>{
    let mut conf: Vec<String> = Vec::new();
    match appname{ 
        "kitty" => 
            conf = gen::generate_hex(&colors, "kitty.conf".to_string()),
        "alacritty" =>
            conf = gen::generate_hex(&colors, "alacritty.yml".to_string()), 
        "i3" => 
            conf = gen::generate_hex(&colors, "i3config".to_string()),
        "eww"=> 
            conf = gen::generate_hex(&colors, "eww.scss".to_string()),
        "sway" => 
            conf = gen::generate_hex(&colors, "sway".to_string()),
        "waybar" => 
            conf = gen::generate_hex(&colors, "waybar.css".to_string()),
        _ => println!("No template for the app {}", appname),
    }
    conf
}