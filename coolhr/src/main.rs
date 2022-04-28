fn main() {
    println!("Cool HR!");
}

fn process(cmd: String) -> (String, String) {
    let mut emp = String::new();
    let mut dept = String::new();

    let mut add_emp = false;
    let mut add_dept = false;
    for tok in cmd.split_whitespace() {
        if add_emp {
            emp.push_str(tok);
        }

        if add_dept {
            dept.push_str(tok);
        }

        add_emp = tok == "Add";
        add_dept = tok == "to";
    }

    (dept, emp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cmd = "Add Tim to Engineering";

        assert_eq!(
            process(cmd.to_string()),
            (String::from("Engineering"), String::from("Tim"))
        );
    }
}
