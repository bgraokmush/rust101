fn main() {
    let mut bird = Bird {
        name: String::from("Eagle"),
        attack_power: 100,
    };
    bird.print_name();

    //if you want to change the name of the bird
    bird.change_name(String::from("Hawk"));

    bird.print_name();
}

struct Bird {
    name: String,
    attack_power: u32,
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }

    fn print_name(&self) {
        println!("Bird name: {}", self.name);
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn is_dangerous(&self) -> bool {
        // if attack power is greater than 50, then it is dangerous
        self.attack_power > 50
    }
}

// like interface in java
trait Animal {
    fn can_fly(&self) -> bool;
    fn print_name(&self);
    fn change_name(&mut self, new_name: String);
    fn is_dangerous(&self) -> bool {
        false // default implementation
    }
}
