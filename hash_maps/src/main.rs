mod company {
    #[derive(Hash, Eq, Clone, Debug)]
    pub struct Employee {
        id: String,
        name: String,
    }
    impl PartialEq for Employee {
        fn eq(&self, other: &Employee) -> bool {
            self.id == other.id && self.name == other.name
        }
    }

    impl Employee {
        pub fn new() -> Employee {
            Employee {
                id: String::new(),
                name: String::new(),
            }
        }
        pub fn from(id: String, name: String) -> Employee {
            Employee { id, name }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Department {
        name: String,
        num: usize,
        capacity: usize,
    }
    impl PartialEq for Department {
        fn eq(&self, other: &Department) -> bool {
            self.name == other.name
        }
    }
    impl Eq for Department {}

    use std::hash::{Hash, Hasher};
    impl Hash for Department {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl Department {
        pub fn new() -> Department {
            Department {
                name: String::new(),
                num: 0,
                capacity: 0,
            }
        }
        pub fn from(name: String, capacity: usize) -> Department {
            Department {
                name,
                capacity,
                num: 0,
            }
        }
    }
    use std::collections::HashMap;
    pub struct Database {
        employee_info: HashMap<Employee, Department>,
        department_info: HashMap<Department, Vec<Employee>>,
    }
    impl Database {
        pub fn insert(&mut self, person: &Employee, place: &mut Department) {
            let person_vec = self.department_info
                .entry(place.clone())
                .or_insert(vec![person.clone()]);
            if !person_vec.contains(person) {
                person_vec.push(person.clone());
            }
            self.employee_info
                .entry(person.clone())
                .or_insert(place.clone());
            place.num = person_vec.len();
        }
        pub fn get_employee_list(&self, place: &Department) -> Option<&Vec<Employee>> {
            self.department_info.get(place)
        }
        pub fn get_department(&self, person: &Employee) -> Option<&Department> {
            self.employee_info.get(person)
        }
        pub fn new() -> Database {
            Database {
                employee_info: HashMap::new(),
                department_info: HashMap::new(),
            }
        }
    }
}
fn main() {
    let mut sails = company::Department::from(String::from("Sails"), 10);

    let john = company::Employee::from(String::from("1024"), String::from("John"));
    let mike = company::Employee::from(String::from("20162180213"), String::from("lcy"));

    let mut db = company::Database::new();

    db.insert(&john, &mut sails);
    db.insert(&mike, &mut sails);

    match db.get_employee_list(&sails) {
        Some(vec) => println!("{:?}", vec),
        None => println!("No one here"),
    }
    match db.get_department(&john) {
        Some(place) => println!("{:?}", place),
        None => println!("No jobs"),
    }
    match db.get_department(&mike) {
        Some(place) => println!("{:?}", place),
        None => println!("No jobs"),
    }

    println!("{:?}", sails);
}
