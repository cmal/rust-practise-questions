struct Hero {
    name: String
}

impl Drop for Hero {
    fn drop(&mut self) {
        println!(
            "Oh no !!! Our hero {} is defeated",
            self.name
        );
    }
}

fn main() {
    let _hero = Hero {
        name: "Tom".to_string()
    };
    
}
