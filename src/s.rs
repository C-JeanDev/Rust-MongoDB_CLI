///
/// A simple menu with three options to choose from.
///
/// 

use terminal_menu::{menu, label, button, run, mut_menu};


fn main() {
    let menu = menu(vec![

        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("terminal-menu"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),

        // button:
        //  exit the menu
        button("Create"),
        button("Read"),
        button("Update"),
        button("Delete"),

    ]);
    run(&menu);

    let mut scelta = mut_menu(&menu).selected_item_name();
    println!("Selected: {}",mut_menu(&menu).selected_item_name());

}
