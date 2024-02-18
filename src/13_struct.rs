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

impl Bird {
    fn print_name(&self) {
        println!("Bird's name is: {}", self.name);
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}
