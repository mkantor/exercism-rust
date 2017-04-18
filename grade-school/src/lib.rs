use std::collections::{BTreeMap, BTreeSet};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Student<'a> {
    name: &'a str,
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Grade {
    number: u8,
}

pub struct School<'a> {
    roster: BTreeMap<Grade, BTreeSet<Student<'a>>>,
}
impl<'a> School<'a> {
    pub fn new() -> Self {
        School { roster: BTreeMap::new() }
    }

    pub fn add(&mut self, grade_number: u8, student_name: &'a str) {
        let grade = Grade { number: grade_number };
        let student = Student { name: student_name };

        self.roster
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student);
    }

    pub fn grades(&self) -> Vec<u8> {
        self.roster.keys().map(|grade| grade.number).collect()
    }

    pub fn grade(&self, grade_number: u8) -> Option<Vec<String>> {
        let grade = Grade { number: grade_number };

        self.roster
            .get(&grade)
            .map(|students| {
                     students
                         .iter()
                         .map(|student| student.name.to_string())
                         .collect()
                 })
    }
}
