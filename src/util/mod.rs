pub fn to_kebab_case(s: String) -> String {
    let mut name = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if i>0 && c>='A' && c<='Z' {
            name = format!("{}-", name);
        }
        name = format!("{}{}", name, c.to_string());
    }
    name.to_lowercase()
}


