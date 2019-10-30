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

use ::log::{info, error};

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
        Err(e) => { error!("{}", e); 0 },
    }
}

fn get_all_selectable() -> impl Iterator<Item=Node> {
    document().query_selector_all("[nav-selectable]").unwrap().iter()
}

fn rebuild_selectable_indexes() {
    let mut i = 0;
    for elem in get_all_selectable() {
        let e: HtmlElement = elem.try_into().unwrap();
        e.set_attribute("nav-index", &i.to_string()).unwrap();
        i += 1;
    }
}

fn focus_nth_selectable(nth: usize) {
    let first_element: HtmlElement = get_all_selectable().nth(nth).unwrap().try_into().unwrap();
    first_element.set_attribute("nav-selected", "true").unwrap();
    first_element.set_attribute("nav-index", &nth.to_string()).unwrap();
    first_element.focus();
}

fn unselect_element() {
    let selected: Element = document().query_selector("[nav-selected=true]").unwrap().unwrap();
    selected.set_attribute("nav-selected", "false").unwrap();
}

fn select_move(direction: DIRECTION) {
    let all_selectable_cnt = get_all_selectable().count();
    let current_idx = get_index_of_current_selected();
    let dir = match direction {
        DIRECTION::UP => -1,
        DIRECTION::DOWN => 1,
    };
    let new_idx = ((current_idx as i32 + dir + all_selectable_cnt as i32) % all_selectable_cnt as i32) as usize;
    unselect_element();
    focus_nth_selectable(new_idx);
    match new_idx {
        0 => softkey_set_label("", "Insert", ""),
        _n => softkey_set_label("", "Toggle", "Delete"),
    }
}

fn add_todo(text: String) {
    let todo_list = document().query_selector("#toDos").unwrap().unwrap();
    let new_todo = document().create_element("SPAN").unwrap();
    let text_node = document().create_text_node(&text);
    new_todo.set_attribute("nav-selectable", "true").unwrap();
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

fn softkey_set_label(left: &str, center: &str, right: &str) {
    document().query_selector("#left").unwrap().unwrap().set_text_content(left);
    document().query_selector("#center").unwrap().unwrap().set_text_content(center);
    document().query_selector("#right").unwrap().unwrap().set_text_content(right);
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

fn event_soft_right() {
    let idx = get_index_of_current_selected();
    if idx == 0 { return };
    select_move(DIRECTION::UP);
    let todo_list = document().query_selector("#toDos").unwrap().unwrap();
    let to_remove = get_all_selectable().nth(idx).unwrap();
    todo_list.as_node().remove_child(&to_remove).unwrap();
    rebuild_selectable_indexes();
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
            "SoftRight" => event_soft_right(),
            "SoftLeft" => info!("SoftLeft pressed"),
            k => info!("Unknown key {}", k),
        }
    });

    stdweb::event_loop();
    Ok(())
}
