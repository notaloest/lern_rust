fn main() {
    println!("Hello, world!");

    use console_menu::{Menu, MenuOption, MenuProps};

    let menu_options = vec![
        MenuOption::new("option 1", || println!("option one!")),
        MenuOption::new("option 2", || println!("option two!")),
        MenuOption::new("option 3", || println!("option three!")),
    ];
    let mut menu = Menu::new(menu_options, MenuProps::default());
    menu.show();
}
