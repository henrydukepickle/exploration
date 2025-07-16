pub mod game {
    use input::get_input;
    use std::collections::HashMap;

    pub type Int = i16;
    pub const MAX: u16 = 3;
    #[derive(Clone, Debug)]
    pub struct Reality {
        pub state: State,
        pub world: World,
    }
    #[derive(Clone, Debug)]
    pub struct State {
        pub pos: Pos,
        inventory: Vec<Item>,
        event_state: Option<EventTree>,
        stack: Vec<Action>,
        event: Option<Event>,
    }
    #[derive(Clone, Debug)]
    pub struct World {
        pub events: HashMap<Pos, Event>,
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        pub preview: String,
        pub tree: EventTree,
    }
    #[derive(Clone, Debug)]
    pub struct EventTree {
        pub node: EventNode,
        pub sub: HashMap<Action, EventTree>,
    }
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Action {
        Continue,
        Simple(String),
        UseItem(u16),
    }
    #[derive(Clone, Debug)]
    pub struct EventNode {
        pub text: String,
        pub items: Vec<Item>,
    }
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Item {
        pub id: u16,
        pub name: String,
        pub item_type: ItemType,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ItemType {
        Weapon(WeaponData),
        Normal,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct WeaponData {
        pub damage: u16,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Pos {
        pub x: Int,
        pub y: Int,
    }

    impl Default for Reality {
        fn default() -> Self {
            Self {
                state: State {
                    pos: Pos { x: 0, y: 0 },
                    inventory: Vec::new(),
                    event_state: None,
                    stack: Vec::new(),
                    event: None,
                },
                world: World {
                    events: HashMap::new(),
                },
            }
        }
    }

    mod input {
        use std::io;
        pub fn get_input(prompt: &str) -> String {
            println!("{}", prompt);
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_goes_into_input_above) => {}
                Err(_no_updates_is_fine) => {}
            }
            input.trim().to_string()
        }
    }

    impl EventTree {
        fn get_node(&self, stack: &Vec<Action>) -> Result<EventTree, ()> {
            let mut curr_tree = self.clone();
            for act in stack {
                curr_tree = curr_tree.sub.get(act).ok_or(())?.clone();
            }
            Ok(curr_tree)
        }
    }

    impl Pos {
        fn bound(&self, max: u16) -> Pos {
            let m = max as i16;
            Pos {
                x: self.x.max(-m).min(m),
                y: self.y.max(-m).min(m),
            }
        }
    }

    impl Reality {
        fn travel(&mut self, dir: String) {
            match dir.as_str() {
                "w" | "W" => {
                    self.state.pos.y += 1;
                }
                "a" | "A" => {
                    self.state.pos.x -= 1;
                }
                "s" | "S" => {
                    self.state.pos.y -= 1;
                }
                "d" | "D" => {
                    self.state.pos.x += 1;
                }
                "f" | "F" => {
                    self.encounter();
                    return;
                }
                _ => {}
            }
            self.state.pos = self.state.pos.bound(MAX);
            self.state.event = self.world.events.get(&self.state.pos).cloned();
            self.state.event_state = None;
        }
        fn encounter(&mut self) {
            if let Some(event) = self.world.events.get(&self.state.pos) {
                self.state.event_state = Some(event.tree.clone());
            }
        }
        fn progress(&mut self, act: Action) {
            if let Some(ev) = &mut self.state.event_state {
                if act == Action::Simple(String::from("q"))
                    || act == Action::Simple(String::from("Q"))
                {
                    self.state.event_state = self.back();
                    return;
                }
                match ev.sub.get_mut(&act) {
                    None => {}
                    Some(sub) => {
                        self.state.stack.push(act);
                        do_node(&mut self.state.inventory, &mut sub.node);
                        *ev = sub.clone();
                    }
                }
            }
        }
        fn get_action(&self, input: String) -> Option<Action> {
            if input == String::from("e") || input == String::from("E") {
                return Some(Action::UseItem(self.choose_item()?.id));
            } else {
                return Some(Action::Simple(input));
            }
        }
        fn choose_item(&self) -> Option<Item> {
            for i in 0..self.state.inventory.len() {
                println!("{i}. {}", self.state.inventory[i].name);
            }
            let inp = get_input("Which Item?   ");
            self.state
                .inventory
                .get(inp.parse::<usize>().ok()?)
                .cloned()
        }
        fn back(&mut self) -> Option<EventTree> {
            self.state.stack.pop()?;
            Some(
                self.state
                    .event
                    .clone()?
                    .tree
                    .get_node(&self.state.stack)
                    .expect("FAILED"),
            )
        }
        fn output(&self) {
            if let Some(ev) = &self.state.event_state {
                println!("{}", ev.node.text);
            } else {
                println!("You are at ({}, {})", self.state.pos.x, self.state.pos.y);
                if let Some(ev) = &self.state.event {
                    println!("{}", ev.preview)
                }
            }
        }
        fn turn(&mut self, input: String) {
            if input == String::from("i") {
                for item in &self.state.inventory {
                    print!("{}", item.name);
                    if let ItemType::Weapon(w) = item.item_type {
                        print!(": {} damage", w.damage);
                    }
                    println!("");
                }
                if self.state.inventory.is_empty() {
                    println!("You have no items.");
                }
            }
            match &self.state.event_state {
                None => self.travel(input),
                Some(x) => {
                    if let Some(action) = self.get_action(input) {
                        self.progress(action)
                    }
                }
            }
        }
        fn input(&mut self) {
            self.output();
            self.turn(get_input(""));
        }
        pub fn run(&mut self) {
            loop {
                self.input();
            }
        }
    }
    fn do_node(inventory: &mut Vec<Item>, node: &mut EventNode) {
        loop {
            match node.items.pop() {
                None => break,
                Some(item) => inventory.push(item),
            }
        }
    }
}
