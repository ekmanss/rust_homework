use std::collections::HashMap;

pub enum Gender {
    Male,
    Female,
}

// `Student` struct now includes a `class_id` field to indicate which class the student belongs to.
pub struct Student {
    id: u32,
    name: String,
    age: u8,
    gender: Gender,
    class_id: Option<u32>,
}

// `Clazz` struct now includes a `students` field to indicate which students belong to the class.
pub struct Clazz {
    id: u32,
    name: String,
    students: Vec<u32>,
}

pub struct StudentManager {
    students: HashMap<u32, Student>,
    classes: HashMap<u32, Clazz>,
}

impl StudentManager {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    // Creates a new student with the given `id`, `name`, `age`, and `gender`.
    pub fn create_student(&mut self, id: u32, name: String, age: u8, gender: Gender) {
        let student = Student {
            id,
            name,
            age,
            gender,
            class_id: None,
        };
        self.students.insert(student.id, student);
    }

    // Creates a new class with the given `id` and `name`.
    pub fn create_class(&mut self, id: u32, name: String) {
        let class = Clazz {
            id,
            name,
            students: Vec::new(),
        };
        self.classes.insert(class.id, class);
    }

    // Assigns a student to a class.
    pub fn assign_student_to_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(student) = self.students.get_mut(&student_id) {
            student.class_id = Some(class_id);
            if let Some(class) = self.classes.get_mut(&class_id) {
                class.students.push(student_id);
            }
        }
    }

    // Removes a student from a class.
    pub fn remove_student_from_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(student) = self.students.get_mut(&student_id) {
            student.class_id = None;
            if let Some(class) = self.classes.get_mut(&class_id) {
                class.students.retain(|&id| id != student_id);
            }
        }
    }

    // Updates the class a student belongs to.
    pub fn update_student_class(&mut self, student_id: u32, new_class_id: u32) {
        if let Some(student) = self.students.get_mut(&student_id) {
            if let Some(old_class_id) = student.class_id {
                if let Some(old_class) = self.classes.get_mut(&old_class_id) {
                    old_class.students.retain(|&id| id != student_id);
                }
            }
            student.class_id = Some(new_class_id);
            if let Some(new_class) = self.classes.get_mut(&new_class_id) {
                new_class.students.push(student_id);
            }
        }
    }
}
