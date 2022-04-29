use std::collections::HashMap;
use std::io;

fn main() {
    println!("== Cool HR! ==");

    let mut cmd = String::new();

    let mut records: HashMap<String, Vec<String>> = HashMap::new();

    show_instructions();

    while cmd.trim() != "q" {
        if cmd.trim() == "?" {
            show_instructions();
        }
        cmd.clear();
        io::stdin().read_line(&mut cmd).expect("Failed to line");

        let (emp, dept) = process(cmd.to_string());

        register(&emp, &dept, &mut records);
    }

    println!("{:?}", records);
}

fn show_instructions() {
    println!("\nInstructions");
    println!("============\n");
    println!("Use `add` command to add new employee.\n\tExample: `Add Sally to Sales`.");
    println!("Use `q` when you're done.");
    println!("Use `?` if you need help.\n");
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

        add_emp = tok.to_lowercase() == "add";
        add_dept = tok.to_lowercase() == "to";
    }

    (emp, dept)
}

fn register(emp: &String, dept: &String, records: &mut HashMap<String, Vec<String>>) {
    if emp != "" && dept != "" {
        let v = records.entry(dept.to_string()).or_insert(Vec::new());
        v.push(emp.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cmd = "Add Tim to Engineering";

        assert_eq!(
            process(cmd.to_string()),
            (String::from("Tim"), String::from("Engineering"))
        );

        let cmd = "Add Kate to Sales";

        assert_eq!(
            process(cmd.to_string()),
            (String::from("Kate"), String::from("Sales"))
        );

        let rob = "Rob";
        let sheryl = "Sheryl";
        let john = "John";
        let steve = "Steve";

        let engg = "Engineering";
        let mktg = "Marketing";

        let mut records: HashMap<String, Vec<String>> = HashMap::new();
        let marketers = vec![sheryl.to_string(), john.to_string(), steve.to_string()];
        records.insert(engg.to_string(), vec![rob.to_string()]);
        records.insert(mktg.to_string(), marketers);

        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let entries = [(rob, engg), (sheryl, mktg), (john, mktg), (steve, mktg)];
        for (emp, dept) in &entries {
            register(&emp.to_string(), &dept.to_string(), &mut results);
        }

        assert_eq!(results, records)
    }
}
