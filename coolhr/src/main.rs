use std::collections::HashMap;

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

fn register(emp: &String, dept: &String, coy: &mut HashMap<String, Vec<String>>) {
    let v = coy.entry(dept.to_string()).or_insert(Vec::new());
    v.push(emp.to_string());
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

        let cmd = "Add Kate to Sales";

        assert_eq!(
            process(cmd.to_string()),
            (String::from("Sales"), String::from("Kate"))
        );

        let rob = "Rob";
        let sheryl = "Sheryl";
        let john = "John";
        let steve = "Steve";

        let engg = "Engineering";
        let mktg = "Marketing";

        let mut coy: HashMap<String, Vec<String>> = HashMap::new();
        let marketers = vec![sheryl.to_string(), john.to_string(), steve.to_string()];
        coy.insert(engg.to_string(), vec![rob.to_string()]);
        coy.insert(mktg.to_string(), marketers);

        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let entries = [
            (rob, engg),
            (sheryl, mktg),
            (john, mktg),
            (steve, mktg),
        ];
        for (emp, dept) in &entries {
            register(&emp.to_string(), &dept.to_string(), &mut results);
        }

        assert_eq!(results, coy)
    }
}
