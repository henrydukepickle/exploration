use input::get_input;
use std::collections::HashMap;

type Int = i16;
const MAX: u16 = 3;
#[derive(Clone, Debug)]
struct Reality {
    state: State,
    world: World,
}
#[derive(Clone, Debug)]
struct State {
    pos: Pos,
    inventory: Vec<Item>,
    event_state: Option<EventTree>,
    stack: Vec<EventTree>,
    event: Option<Event>,
}
#[derive(Clone, Debug)]
struct World {
    events: HashMap<Pos, Event>,
}
#[derive(Clone, Debug)]
struct Event {
    preview: String,
    tree: EventTree,
}
#[derive(Clone, Debug)]
struct EventTree {
    node: EventNode,
    sub: HashMap<Action, EventTree>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Action {
    Continue,
    Simple(String),
    UseItem(Item), //if the item was used
}
#[derive(Clone, Debug)]
struct EventNode {
    text: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Item {
    id: u16,
    name: String,
    item_type: ItemType,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ItemType {
    Weapon(WeaponData),
    Normal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct WeaponData {
    damage: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: Int,
    y: Int,
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
            if act == Action::Simple(String::from("q")) || act == Action::Simple(String::from("Q"))
            {
                self.state.event_state = self.back();
                return;
            }
            match ev.sub.get(&act) {
                None => {}
                Some(sub) => {
                    self.state.stack.push(ev.clone());
                    *ev = sub.clone();
                }
            }
        }
    }
    fn get_action(&self, input: String) -> Option<Action> {
        if input == String::from("e") || input == String::from("E") {
            return Some(Action::UseItem(self.choose_item()?));
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
        self.state.stack.pop()
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
    fn run(&mut self) {
        loop {
            self.input();
        }
    }
}

fn make_reality() -> Reality {
    let mut evs = HashMap::new();
    let ev1 = EventTree {
        node: EventNode {
            text: String::from("TEST"),
        },
        sub: HashMap::new(),
    };
    let ev2 = EventTree {
        node: EventNode {
            text: String::from("you found the individual"),
        },
        sub: HashMap::new(),
    };
    let ev3 = EventTree {
        node: EventNode {
            text: String::from("PRESS k TO GO INSIDE"),
        },
        sub: HashMap::from([
            (Action::Simple(String::from("k")), ev1),
            (
                Action::UseItem(Item {
                    id: 0,
                    name: String::from("key"),
                    item_type: ItemType::Normal,
                }),
                ev2,
            ),
        ]),
    };
    evs.insert(
        Pos { x: 1, y: 0 },
        Event {
            preview: String::from("There is a guy"),
            tree: ev3,
        },
    );
    Reality {
        world: World { events: evs },
        state: State {
            pos: Pos { x: 0, y: 0 },
            inventory: vec![Item {
                name: String::from("key"),
                id: 0,
                item_type: ItemType::Normal,
            }],
            event_state: None,
            stack: Vec::new(),
            event: None,
        },
    }
}

fn main() {
    make_reality().run();
}
