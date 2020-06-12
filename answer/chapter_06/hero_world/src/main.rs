struct Hero {
    name: String
}

struct World {
    hero: Box<Hero>
}

impl Drop for Hero {
    fn drop(&mut self) {
        println!(
            "Oh no !!! Our hero {} is defeated",
            self.name
        );
    }
}

impl Drop for World {
    fn drop(&mut self) {
        println!("The world ends here !!!");
    }
}

fn main() {
    let hero = Hero {
        name: "Tom".to_string()
    };
    let _world = World {
        hero: Box::new(hero)
    };
    
}
