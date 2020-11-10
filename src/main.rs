#![windows_subsystem = "windows"]

use cursive::views::*;
use cursive::traits::*;
use cursive::Cursive;
use cursive::view::{IntoBoxedView, ViewWrapper};
use cursive::align::{Align, HAlign};
use cursive::wrap_impl;

struct ShipHull {
    class: String,
    size: String,
    weapons: Vec<String>,
    view: LinearLayout,
}

impl ShipHull {
    fn new(class: impl Into<String> + Clone, size: impl Into<String> + Clone, weapons: Vec<String>) -> Self {
        let mut equipment_view = LinearLayout::horizontal();
        for weapon in &weapons {
            equipment_view.add_child(TextView::new(weapon).align(Align::center()).full_width())
        }

        let hull_view = LinearLayout::vertical()
            .child(TextView::new(format!("Hull: {}-class {}", class.clone().into(), size.clone().into())))
            .child(equipment_view);

        Self { class: class.into(), size: size.into(), weapons, view: hull_view }
    }
}

impl Clone for ShipHull {
    fn clone(&self) -> Self {
        Self::new(&self.class, &self.size, self.weapons.clone())
    }
}

impl ViewWrapper for ShipHull {
    wrap_impl!(self.view: LinearLayout);
}

struct Ship {
    name: String,
    hull: ShipHull,
    view: Panel<ShipHull>,
}

impl Ship {
    fn new(name: impl Into<String> + Clone, hull: ShipHull) -> Self {
        let view = Panel::new(hull.clone())
            .title(name.clone())
            .title_position(HAlign::Left);
        Ship { name: name.into(), hull, view }
    }

    fn dialog_add(ctx: &mut Cursive) {
        fn ok(ctx: &mut Cursive, name: &str) {
            ctx.call_on_name("Ship list", |list: &mut ListView| {
                //list.add_child(TextView::new(name));
                list.add_child("test", TextView::new("test"));
            });
            ctx.pop_layer();
        }

        ctx.add_layer(
            Dialog::around(
                EditView::new()
                    .on_submit(ok)
                    .with_name("Add ship dialog")
            )
                .title("Add ship")
                .button("Add", |ctx| {
                    let new_ship_name = ctx.call_on_name("Add ship dialog", |data: &mut EditView| { data.get_content() }).unwrap();
                    ok(ctx, &new_ship_name)
                })
                .dismiss_button("Cancel")
        )
    }
}

impl Clone for Ship {
    fn clone(&self) -> Self {
        Self::new(&self.name, self.hull.clone())
    }
}

impl ViewWrapper for Ship {
    wrap_impl!(self.view: Panel<ShipHull>);
}

fn main() {
    let mut ctx = cursive::default();

    ctx.add_global_callback('q', |s| s.quit());

    ctx.add_layer(LinearLayout::vertical()
        .child(Ship::new("Weaponship1", ShipHull::new("Alpha", "Frigate", vec!["Weapon".to_string()])))
        .child(Ship::new("Weaponship2", ShipHull::new("Beta", "Frigate", vec!["Weapon1".to_string(), "Weapon2".to_string(), "Weapon3".to_string()])))
        .child(Ship::new("Weaponship3", ShipHull::new("Gamma", "Frigate", vec!["WeaponX".to_string(), "WeaponY".to_string()])))
        .child(Button::new("Add ship", Ship::dialog_add))
        .with_name("Ship list")
        .full_width()
    );

    ctx.run();
}