use std::collections::HashMap;
use std::io;

fn main() {
    println!("== Cool HR! ==");

    let mut cmd = String::new();

    let mut records: HashMap<String, Vec<String>> = HashMap::new();

    show_instructions();

    while cmd.trim() != "q" {
        cmd.clear();

        io::stdin().read_line(&mut cmd).expect("Failed to line");

        if cmd.trim() == "?" {
            show_instructions();
            continue;
        }

        if cmd.to_lowercase().starts_with("add") {
            let (emp, dept) = process_add(cmd.to_string());

            register(&emp, &dept, &mut records);
            continue;
        }

        if cmd.to_lowercase().starts_with("list") {
            let mut depts: Vec<&String> = Vec::new();
            let dept = process_list(cmd.to_string());

            if dept.to_lowercase() == "all" {
                depts = records.keys().collect();
                depts.sort();
            } else {
                depts.push(&dept);
            }

            for dept in depts {
                let emps = sorted_employees(&dept, &records);
                println!("{} Department", dept);
                println!("================");
                let mut count = 0;
                for emp in emps {
                    count += 1;
                    println!(" {}. {}", count, emp);
                }
                println!("================");
            }
            continue;
        }

        if cmd.trim() == "q" {
            continue;
        }

        println!("Invalid command! Type `?` if you need help.")
    }
}

fn show_instructions() {
    println!("\nInstructions");
    println!("============\n");
    println!("Use `add` command to add new employee.\n\tExample: `Add Sally to Sales`.");
    println!("Use `list` command to show list of employees.\n\tExample: `List Sales`.");
    println!("Use `q` when you're done.");
    println!("Use `?` if you need help.\n");
}

fn process_add(cmd: String) -> (String, String) {
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

fn process_list(cmd: String) -> String {
    let mut dept = String::new();
    let mut list_emp = false;
    for tok in cmd.split_whitespace() {
        if list_emp {
            dept = tok.to_string();
        }

        list_emp = tok.to_lowercase() == "list";
    }

    dept
}

fn sorted_employees(dept: &String, records: &HashMap<String, Vec<String>>) -> Vec<String> {
    match records.get(dept) {
        Some(emps) => {
            let mut emps_copy = emps.clone();
            emps_copy.sort();

            emps_copy
        }
        None => {
            println!("{:?} not found!", dept);
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cmd = "Add Tim to Engineering";

        assert_eq!(
            process_add(cmd.to_string()),
            (String::from("Tim"), String::from("Engineering"))
        );

        let cmd = "Add Kate to Sales";

        assert_eq!(
            process_add(cmd.to_string()),
            (String::from("Kate"), String::from("Sales"))
        );

        let mut records: HashMap<String, Vec<String>> = HashMap::new();
        let marketers = vec![
            "Sheryl".to_string(),
            "John".to_string(),
            "Steve".to_string(),
        ];
        records.insert("Engineering".to_string(), vec!["Rob".to_string()]);
        records.insert("Marketing".to_string(), marketers);

        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let commands = [
            "Add Rob to Engineering",
            "Add Sheryl to Marketing",
            "Add John to Marketing",
            "Add Steve to Marketing",
        ];
        for cmd in &commands {
            let (emp, dept) = process_add(cmd.to_string());
            register(&emp.to_string(), &dept.to_string(), &mut results);
        }

        assert_eq!(results, records);

        let mut records: HashMap<String, Vec<String>> = HashMap::new();
        let marketers = vec![
            "Sheryl".to_string(),
            "John".to_string(),
            "Steve".to_string(),
        ];
        let engineers = vec!["Rob".to_string(), "Rus".to_string(), "Filipo".to_string()];
        records.insert("Engineering".to_string(), engineers);
        records.insert("Marketing".to_string(), marketers);

        let cmd = String::from("List Marketing");
        let dept = process_list(cmd);
        assert_eq!(dept, "Marketing");

        let sorted_marketers = vec!["John", "Sheryl", "Steve"];
        assert_eq!(sorted_employees(&dept, &records), sorted_marketers);
    }
}
