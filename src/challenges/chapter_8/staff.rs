use std::collections::HashMap;

pub fn runner(){
    // TODO: console app to allow people to add employees and see the employees in a department.
}

pub struct company {
    departments: HashMap<String, Vec<String>>
}

impl company {
    pub fn new() -> company {
        company {
            departments: HashMap::new()
        }
    }

    pub fn add_staff_member(&mut self, name: String, department: String) {
        self.departments.entry(name).or_insert(vec![department]);
    }

    pub fn get_staff_in_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }
}