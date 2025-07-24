use std::collections::HashMap;

pub struct Organisation {
    pub departments: Vec<String>,
    pub departments_names: HashMap<String, Vec<String>>,
}

impl Organisation {
    pub fn new() -> Organisation {
        Organisation {
            departments: vec![],
            departments_names: HashMap::new(),
        }
    }
    pub fn add_employee<'a, 'b>(&mut self, name: &'a str, department: &'a str) {
        let dep_names = self
            .departments_names
            .entry(department.to_string())
            .or_insert(Vec::new());
        dep_names.push(name.to_string());
        dep_names.sort();

        if !self.departments.contains(&department.to_string()) {
            self.departments.push(department.to_string());
        }
    }

    pub fn print_dep(&self, department: &str) {
        let dep_names: &Vec<String> = match self.departments_names.get(&department.to_string()) {
            Some(dep_names) => dep_names,
            None => &vec![(String::from("Did not find department: ") + department)],
        };
        println!("{:?}", dep_names)
    }

    pub fn print_all(&self) {
        for dep in self.departments.iter() {
            println!("{dep}: {:?}", self.departments_names.get(dep).unwrap());
        }
    }
}
