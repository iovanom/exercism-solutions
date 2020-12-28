use std::collections::HashMap;

pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade_students = self.students.entry(grade).or_insert(vec![]);
        grade_students.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res: Vec<u32> = self.students.keys().copied().collect();
        res.sort();
        res
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students.get(&grade).map(|v| {
            let mut res = v.clone();
            res.sort();
            res
        })
    }
}
