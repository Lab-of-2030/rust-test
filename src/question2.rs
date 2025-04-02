// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。

#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    age: u8,
    score: f32,
}

impl Student {
    // Create a new Student instance
    pub fn new(name: &str, age: u8, score: f32) -> Self {
        Self {
            name: name.to_string(),
            age,
            score,
        }
    }

    // Display the student's information
    pub fn show(&self) -> String {
        format!("Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score)
    }

    // Check if the student passed
    pub fn is_passed(&self) -> bool {
        self.score >= 60.0
    }

    // Run function for external use
    pub fn run(&self) {
        println!("{}", self.show());
        if self.is_passed() {
            println!("Status: Passed");
        } else {
            println!("Status: Failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_student() {
        let student = Student::new("Alice", 18, 95.5);
        assert_eq!(student.name, "Alice");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 95.5);
    }

    #[test]
    fn test_show_student() {
        let student = Student::new("Bob", 20, 85.0);
        assert_eq!(student.show(), "Name: Bob, Age: 20, Score: 85.0");
    }

    #[test]
    fn test_is_passed_true() {
        let student = Student::new("Charlie", 22, 75.0);
        assert!(student.is_passed());
    }

    #[test]
    fn test_is_passed_false() {
        let student = Student::new("David", 19, 50.0);
        assert!(!student.is_passed());
    }

    #[test]
    fn test_edge_case_passed() {
        let student = Student::new("Eve", 21, 60.0);
        assert!(student.is_passed());
    }

    #[test]
    fn test_edge_case_failed() {
        let student = Student::new("Frank", 23, 59.9);
        assert!(!student.is_passed());
    }
}
