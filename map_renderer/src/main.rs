mod map;

use minidom::Element;
use std::fs;

const DEFAULT_NAME_SPACE: &str = "minidom";

fn main() {
    let xml_string = fs::read_to_string("test_lab.xml").expect("File not found");

    let xml_root: Element = xml_string.parse().unwrap();
    // let version = xml_root.attr("version").unwrap();
    let xml_base_unit_svg = xml_root
        .get_child("BaseUnitInSvg", DEFAULT_NAME_SPACE)
        .unwrap();
    let baseunit_text = xml_base_unit_svg.text();
    let base_unit_in_svg: usize = baseunit_text.parse().unwrap();

    let (map_width, map_height) = read_map_definitions(&xml_root, base_unit_in_svg);

    let room_list: Vec<Room> = load_all_rooms(&xml_root, base_unit_in_svg);

    // TODO RESUME: create the render module and do the lead in and lead out of that.
    map::render(map_width, map_height, room_list);
}

// TODO move this to its own file(module?)
pub struct Room {
    m_start_x: usize,
    m_start_y: usize,
    m_width: usize,
    m_height: usize,
    door_list: Vec<Door>,
    gm_note: String,
    // TODO window list
}

pub struct Door {
    m_start_x: usize,
    m_start_y: usize,
    m_end_x: usize,
    m_end_y: usize,
}

fn read_map_definitions(xml_root: &Element, base_unit_in_svg: usize) -> (usize, usize) {
    let xml_map_definitions = xml_root
        .get_child("MapDefinitions", DEFAULT_NAME_SPACE)
        .unwrap();
    let xml_map_width = xml_map_definitions
        .get_child("MapWidth", DEFAULT_NAME_SPACE)
        .unwrap();
    let xml_map_height = xml_map_definitions
        .get_child("MapHeight", DEFAULT_NAME_SPACE)
        .unwrap();
    let width: usize = xml_map_width.text().parse().unwrap();
    let height: usize = xml_map_height.text().parse().unwrap();
    (width * base_unit_in_svg, height * base_unit_in_svg)
}

/**
 * load all doors, given in xml_room and return as a vector.
 */
fn load_all_doors_in_a_room(xml_room: &Element, base_unit_in_svg: usize) -> Vec<Door> {
    println!("load_door()  elment name: {}", xml_room.name());
    let mut door_list: Vec<Door> = Vec::new();
    for xml_child in xml_room.children() {
        if xml_child.name() == "Door" {
            let door = load_a_door(xml_child, base_unit_in_svg);
            door_list.push(door);
        }
    }
    door_list
}

fn load_a_door(xml_door: &Element, base_unit_in_svg: usize) -> Door {
    // TODO verify the name of the element is  Door
    // TODO handle if an attribute is missing; Report the error.
    let xpos_str = xml_door.attr("startx").unwrap();
    let ypos_str = xml_door.attr("starty").unwrap();
    let endx_str = xml_door.attr("endx").unwrap();
    let endy_str = xml_door.attr("endy").unwrap();
    let xpos: usize = xpos_str.parse().unwrap();
    let ypos: usize = ypos_str.parse().unwrap();
    let endx: usize = endx_str.parse().unwrap();
    let endy: usize = endy_str.parse().unwrap();

    Door {
        m_start_x: xpos * base_unit_in_svg,
        m_start_y: ypos * base_unit_in_svg,
        m_end_x: endx * base_unit_in_svg,
        m_end_y: endy * base_unit_in_svg,
    }
}

fn load_a_room(xml_room: &Element, base_unit_in_svg: usize) -> Room {
    let xpos_str = xml_room.attr("xpos").unwrap();
    let ypos_str = xml_room.attr("ypos").unwrap();
    let width_str = xml_room.attr("width").unwrap();
    let height_str = xml_room.attr("height").unwrap();
    let xpos: usize = xpos_str.parse().unwrap();
    let ypos: usize = ypos_str.parse().unwrap();
    let width: usize = width_str.parse().unwrap();
    let height: usize = height_str.parse().unwrap();

    Room {
        m_start_x: xpos * base_unit_in_svg,
        m_start_y: ypos * base_unit_in_svg,
        m_width: width * base_unit_in_svg,
        m_height: height * base_unit_in_svg,
        gm_note: get_text_from_child_element(&xml_room, "gm_note"),
        door_list: load_all_doors_in_a_room(xml_room, base_unit_in_svg),
    }
}

/**
 * load all rooms, given in xml_root return as a vector.
 */
fn load_all_rooms(xml_root: &Element, base_unit_in_svg: usize) -> Vec<Room> {
    let mut room_list: Vec<Room> = Vec::new();
    for xml_child in xml_root.children() {
        if xml_child.name() == "Room" {
            let room = load_a_room(xml_child, base_unit_in_svg);
            room_list.push(room);
        }
    }
    room_list
}

// TODO I could also return an Option, where it would be None if the child does not exist.
fn get_text_from_child_element(xml_element: &Element, child_element_name: &str) -> String {
    match get_first_xml_child_by_tag_name(&xml_element, child_element_name) {
        Some(xml_child) => xml_child.text(),
        None => "".to_string(),
    }
}

// For a good explanation of lifetime, please see: https://www.youtube.com/watch?v=1QoT9fmPYr8
fn get_first_xml_child_by_tag_name<'a>(
    xml_element: &'a minidom::Element,
    child_element_name: &str,
) -> Option<&'a minidom::Element> {
    for xml_child in xml_element.children() {
        if xml_child.name() == child_element_name {
            return Some(xml_child);
        }
    }
    None
}
