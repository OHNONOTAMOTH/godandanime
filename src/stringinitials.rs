pub fn parse(get: String) -> String {
    let mut initials: String = "".to_string();
    let mut index = 0;
    let hh = &get
                .chars()
                .nth(0)
                .unwrap().
                to_string()[..];
        initials.push(hh.to_owned().chars().nth(0).unwrap());
    for i in get.chars() {
        index += 1;
        if i == ' ' {
            let h = &get
                .chars()
                .nth(index)
                .unwrap().
                to_string()[..];
            initials.push(h.to_owned().chars().nth(0).unwrap());
            }
    }
    return initials;
}
