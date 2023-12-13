use crate::common;

pub fn generate_hex(colors: &Vec<String>, template_name: String) -> Vec<String> {
    let mut config: Vec<String> = vec![String::new(); 44];
    let mut count = 0;
    if let Ok(lines) 
        = common::read_lines(format!("{}/.config/clrgen/templates/{}", common::get_home_dir(), template_name)) {
        for line in lines {
            if let Ok(mut conf_line) = line {
                for i in (0..18).rev(){
                    conf_line = conf_line.replace(&format!("${}", i),&colors[i]);
                }
                config[count] = conf_line;
                count +=1;
            }
        }
    }
    config
}