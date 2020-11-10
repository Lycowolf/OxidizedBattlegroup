#![windows_subsystem = "windows"]

use cursive::views::*;
use cursive::traits::*;
use cursive::Cursive;
use cursive::view::{IntoBoxedView, ViewWrapper};
use cursive::align::{Align, HAlign};
use cursive::wrap_impl;

struct Weapon {
    view: ResizedView<EditView>
}

impl Weapon {
    fn new(name: &str) -> Self {
        Weapon { view: EditView::new().content(name).fixed_width(15) }
    }

    fn serialize(&self) -> String {
        let x = self.view.get_inner().get_content();
        format!("\"{}\"", x.to_string())
    }
}

impl ViewWrapper for Weapon {
    wrap_impl!(self.view: ResizedView<EditView>);
}


struct Hull {
    view: LinearLayout,
}

impl Hull {
    fn new(name: &str, weapons: Vec<Weapon>) -> Self {
        let mut weapons_layout = LinearLayout::horizontal();
        for w in weapons {
            weapons_layout.add_child(w);
        }

        let title_layout = LinearLayout::horizontal()
            .child(TextView::new("Hull class: "))
            .child(EditView::new().content(name).full_width());

        let view = LinearLayout::vertical()
            .child(title_layout)
            .child(weapons_layout);

        Self { view }
    }

    fn serialize(&self) -> String {
        let title_view = self.view.get_child(0).unwrap().as_any().downcast_ref::<LinearLayout>().unwrap();
        let weapons_view = self.view.get_child(1).unwrap().as_any().downcast_ref::<LinearLayout>().unwrap();

        let hull_class_view = title_view.get_child(1).unwrap().as_any().downcast_ref::<ResizedView<EditView>>().unwrap().get_inner();
        let name = hull_class_view.get_content().to_string();

        let mut weapons = Vec::new();
        for i in 0..weapons_view.len() {
            let weapon = weapons_view.get_child(i).unwrap().as_any().downcast_ref::<Weapon>().unwrap();
            weapons.push(weapon);
        }
        let weapons = weapons.iter().map(|i| i.serialize()).collect::<Vec<String>>().join(", ");
        format!("{}: ({})", name, weapons)
    }
}

impl ViewWrapper for Hull {
    wrap_impl!(self.view: LinearLayout);
}

struct Ship {
    view: Panel<LinearLayout>,
}

impl Ship {
    fn new(name: &str, hull: Hull) -> Self {
        let title_view = LinearLayout::horizontal()
            .child(TextView::new("Name: "))
            .child(EditView::new().content(name).full_width());

        let ship_view = LinearLayout::vertical()
            .child(title_view)
            .child(TextView::new("Alpha: the Sigma-class Frigate"))
            .child(hull);

        let view = Panel::new(ship_view).title("Ship").title_position(HAlign::Left);

        Self { view }
    }

    fn serialize(&self) -> String {
        let title_view = self.view.get_inner().get_child(0).unwrap().as_any().downcast_ref::<LinearLayout>().unwrap();
        let hull_view = self.view.get_inner().get_child(2).unwrap().as_any().downcast_ref::<Hull>().unwrap();

        let name = title_view.get_child(1).unwrap().as_any().downcast_ref::<ResizedView<EditView>>().unwrap().get_inner().get_content().to_string();
        let hull = hull_view.serialize();

        format!("Ship: Name: {}, Hull: {})", name, hull)
    }
}

impl ViewWrapper for Ship {
    wrap_impl!(self.view: Panel<LinearLayout>);
}

fn main() {
    let mut ctx = cursive::default();

    let root_window = Ship::new(
        "My Ship",
        Hull::new(
            "HullName",
            vec![Weapon::new("Weapon1"), Weapon::new("Weapon2")],
        ),
    ).with_name("root");

    ctx.add_global_callback('q', |s| s.quit());

    ctx.add_layer(LinearLayout::vertical()
        .child(root_window)
        .child(Button::new("Quit", |s| s.quit()))
    );

    ctx.run();

    ctx.call_on_name("root", |root: &mut Ship| { println!("{}", root.serialize()) });
}