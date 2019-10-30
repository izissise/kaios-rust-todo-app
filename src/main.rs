extern crate stdweb;
extern crate failure;

use failure::Fallible;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    HtmlElement,
    Element,
    html_element::InputElement,
    Node,
    document,
};

use stdweb::web::event::{
    KeyPressEvent,
};

// use stdweb::web::html_element::InputElement;

use ::log::{info};

mod log;
use crate::log::init_logger;

enum DIRECTION {
    UP,
    DOWN,
}

fn get_index_of_current_selected() -> usize {
    let selected: Element = document().query_selector("[nav-selected=true]").unwrap().unwrap();
    match selected.get_attribute("nav-index").unwrap().parse::<usize>() {
        Ok(i) => i,
        Err(_e) => 0,
    }
}

fn get_all_selectable() -> impl Iterator<Item=Node> {
    document().query_selector_all("[nav-selectable]").unwrap().iter()
}

fn focus_nth_selectable(nth: usize) {
    let first_element: HtmlElement = get_all_selectable().nth(nth).unwrap().try_into().unwrap();
    first_element.set_attribute("nav-selected", "true").unwrap();
    first_element.set_attribute("nav-index", "0").unwrap();
    first_element.focus();
}

fn select_move(direction: DIRECTION) {
    let all_selectable_cnt = get_all_selectable().count();
    let current_idx = get_index_of_current_selected();
    let dir = match direction {
        DIRECTION::UP => 1,
        DIRECTION::DOWN => -1,
    };
    let new_idx = ((current_idx as i32 + dir + all_selectable_cnt as i32) % all_selectable_cnt as i32) as usize;
    focus_nth_selectable(new_idx);
}

fn add_todo(text: String) {
    let todo_list = document().query_selector("#toDos").unwrap().unwrap();
    let new_todo = document().create_element("SPAN").unwrap();
    let text_node = document().create_text_node(&text);
    new_todo.set_attribute("nav-selected", "true").unwrap();
    new_todo.append_child(&text_node);
    todo_list.append_child(&new_todo);
}

fn toggle_todo(element: HtmlElement) {
    let class_list = element.class_list();
    match class_list.contains("completed") {
        true => class_list.remove("completed").unwrap(),
        false => class_list.add("completed").unwrap(),
    };
}

fn event_enter() {
    let idx = get_index_of_current_selected();
    match idx {
        0 => {
            let input: InputElement = get_all_selectable().nth(0).unwrap().try_into().unwrap();
            add_todo(input.raw_value());
            input.set_raw_value("");
        },
        n => toggle_todo(get_all_selectable().nth(n).unwrap().try_into().unwrap()),
    }
}

fn main() -> Fallible<()> {
    // Initialize the logger
    init_logger()?;
    info!("Logger initialized");
    stdweb::initialize();
    focus_nth_selectable(0);

    document().add_event_listener(move |event: KeyPressEvent| {
        match event.key().as_str() {
            "Enter" => event_enter(),
            "ArrowDown" => select_move(DIRECTION::DOWN),
            "ArrowUp" => select_move(DIRECTION::UP),
            "SoftRight" => info!("SoftRight pressed"),
            "SoftLeft" => info!("SoftLeft pressed"),
            k => info!("Unknown key {}", k),
        }
    });

    stdweb::event_loop();
    Ok(())
}
