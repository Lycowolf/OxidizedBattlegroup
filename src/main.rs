use druid::widget::{Align, Flex, Label, List};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc, WidgetExt};
use druid::im::Vector;

#[derive(Clone, Data, Lens)]
struct Battlegroup {
    name: String,
    ships: Vector<Ship>,
}

#[derive(Clone, Data, Lens)]
struct Ship {
    name: String,
    traits: Vector<Trait>,
    weapons: Vector<Weapon>,
    systems: Vector<System>,
    wings: Vector<Wing>,
    escorts: Vector<Escort>,
}

impl Ship {
    fn widget() -> Flex<Self> {
        let traits = List::new(|| { Label::new(|data: &Trait, _: &Env| { data.name.to_string() }) }).lens(Ship::traits);
        let weapons = List::new(|| { Label::new(|data: &Weapon, _: &Env| { data.name.to_string() }) }).lens(Ship::weapons);
        let systems = List::new(|| { Label::new(|data: &System, _: &Env| { data.name.to_string() }) }).lens(Ship::systems);
        let wings = List::new(|| { Label::new(|data: &Wing, _: &Env| { data.name.to_string() }) }).lens(Ship::wings);
        let escorts = List::new(|| { Label::new(|data: &Escort, _: &Env| { data.name.to_string() }) }).lens(Ship::escorts);

        Flex::column()
            .with_child(Label::new(|data: &String, _: &Env| { data.to_string() }).lens(Ship::name))
            .with_child(Flex::row()
                .with_child(traits)
                .with_child(weapons)
                .with_child(systems)
                .with_child(wings)
                .with_child(escorts)
            )
    }
}

#[derive(Clone, Data, Lens, Debug)]
struct Trait {
    name: String,
}

#[derive(Clone, Data, Lens, Debug)]
struct Weapon {
    name: String,
}

#[derive(Clone, Data, Lens, Debug)]
struct System {
    name: String,
}

#[derive(Clone, Data, Lens, Debug)]
struct Wing {
    name: String,
}

#[derive(Clone, Data, Lens, Debug)]
struct Escort {
    name: String,
}

fn mock_generate_ship(name: String) -> Ship {
    Ship {
        name: name.to_string(),
        traits: vec![Trait { name: format!("Trait [{}]", name) }].into(),
        weapons: vec![Weapon { name: format!("Weapon [{}]", name) }].into(),
        systems: vec![System { name: format!("System [{}]", name) }].into(),
        wings: vec![Wing { name: format!("Wing [{}]", name) }].into(),
        escorts: vec![Escort { name: format!("Escort [{}]", name) }].into(),
    }
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title("Oxidized Battlegroup".to_string());

    // create the initial app state
    let mut ships = Vector::new();
    for i in 1..=3 {
        ships.push_back(mock_generate_ship(format!("Carrier {}", i)))
    }
    let initial_state = Battlegroup {
        name: "New Battlegroup".into(),
        ships,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<Battlegroup> {
    // a label that will determine its text based on the current app data.
    let battlegroup_name = Label::new(|name: &String, _env: &Env| name.to_string()).lens(Battlegroup::name);
    let ship_list = List::new(|| {
        Ship::widget()
    }).lens(Battlegroup::ships);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(battlegroup_name)
        .with_child(ship_list);

    // center the two widgets in the available space
    Align::centered(layout)
}