use std::fmt;

struct Student {
    roll_number: String,
    name: String,
    age: u8
}

impl fmt::Debug for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Student")
            .field("_", &self.name)
            .field("Age", &self.age)
            .field("No", &self.roll_number)
            .finish()
        // write!(f.buf, "Debug: {}[{}]<{}>", self.name, self.age, self.roll_number);
    }
}

impl Drop for Student {
    fn drop(&mut self) {
        println!(
            "Roll number {} has name {} with age {} and is a {}",
            self.roll_number,
            self.name,
            self.age,
            if self.age < 18 { "junior" } else { "senoir" }
        );
    }
}

fn main() {
    let mike = Student {
        roll_number: "134135".to_string(),
        name: "Mike".to_string(),
        age: 32
    };

    let john = Student {
        roll_number: "152342".to_string(),
        name: "John".to_string(),
        age: 17
    };

    println!("{:?}", john);
    println!("{:?}", mike);
}
