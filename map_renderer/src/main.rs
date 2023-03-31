mod map_to_svg;

use minidom::Element;
use std::env;
use std::fs;
use std::fs::File;

const APPLICATION_VERSION: &str = "0.3.1";

const DEFAULT_NAME_SPACE: &str = "minidom";
const DOOR_HINGE_LEFT: bool = true;
const DOOR_HINGE_RIGHT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut args_iterator = args.iter();

    let mut local_configuration = ConfigurationInformation::new();

    let mut door_mode = false;

    // Skip the application name thing ($0)
    args_iterator.next();

    loop {
        match args_iterator.next() {
            Some(parameter) => {
                match parameter.as_str() {
                    "--door-mode" => {
                        door_mode = true;
                    }
                    "--door-sections" =>
                        local_configuration.set_door_sections(args_iterator.next()),
                    "--door-width" => local_configuration.set_door_width(args_iterator.next()),
                    "--help" => show_help(),
                    "--input-file" => local_configuration.set_input_file(args_iterator.next()),
                    "--gm-map-file" => local_configuration.set_gm_map_file(args_iterator.next()),
                    "--player-map-file" =>
                        local_configuration.set_player_map_file(args_iterator.next()),
                    "--start-x" => local_configuration.set_start_x(args_iterator.next()),
                    "--start-y" => local_configuration.set_start_y(args_iterator.next()),
                    "--version" => println!("{}", APPLICATION_VERSION),
                    _ => println!("EEE unknown option: '{}'", parameter),
                }
            }
            None => {
                break;
            }
        }
    }

    if door_mode {
        // TODO if there are two sections then split the door.

        let mut file_handle = File::create("tmp_door.svg").expect(
            "Error encountered while creating file!"
        );
        let full_door_width = local_configuration.get_door_width();
        let door_sections_number = local_configuration.get_door_sections();

        map_to_svg::lead_in(&mut file_handle, full_door_width + 200, full_door_width + 400);

        let endx_first_door = full_door_width / door_sections_number;
        let endy_first_door: usize = 100;
        let start_x = 10;
        let door = Door {
            m_start_x: start_x,
            m_start_y: endy_first_door,
            m_end_x: start_x+endx_first_door,
            m_end_y: endy_first_door,
        };
        map_to_svg::render_door(&mut file_handle, &door, local_configuration.get_start_x(), local_configuration.get_start_y(), DOOR_HINGE_LEFT);

        if door_sections_number == 2 {
            let right_door = Door {
                m_start_x: start_x+endx_first_door,
                m_start_y: endy_first_door,
                m_end_x: start_x+full_door_width,
                m_end_y: endy_first_door,
            };
            map_to_svg::render_door(&mut file_handle, &right_door, local_configuration.get_start_x(), local_configuration.get_start_y(), DOOR_HINGE_RIGHT);
        }

        map_to_svg::lead_out(&mut file_handle);
    } else {
        let xml_string = fs
            ::read_to_string(local_configuration.get_input_file_name())
            .expect("File not found");

        let xml_root: Element = xml_string.parse().unwrap();
        // let version = xml_root.attr("version").unwrap();
        let xml_base_unit_svg = xml_root.get_child("BaseUnitInSvg", DEFAULT_NAME_SPACE).unwrap();
        let baseunit_text = xml_base_unit_svg.text();
        let base_unit_in_svg: usize = baseunit_text.parse().unwrap();

        let (map_width, map_height) = read_map_definitions(&xml_root, base_unit_in_svg);

        let room_list: Vec<Room> = load_all_rooms(&xml_root, base_unit_in_svg);

        // TODO RESUME: create the render module and do the lead in and lead out of that.
        map_to_svg::render(
            local_configuration.get_player_map_file_name(),
            local_configuration.get_gm_map_file_name(),
            map_width,
            map_height,
            room_list
        );
    }
}

fn show_help() {
    println!("Render svg map from xml definition file - help text");
    println!("  --door-mode                   : only generate the code for a door");
    println!("  --door-sections <units>       : door units 1 or 2, default 1");
    println!("  --door-width <units>          : door width in units");
    println!("  --gm-map-file <file_name>     : set the output file name, for the gm map");
    println!("  --help                        : this text");
    println!("  --input-file  <file_name>     : set the input file name");
    println!("  --player-map-file <file_name> : set the output file name, for the gm map");
    println!("  --start-x <x-coord>           : ");
    println!("  --start-y <y-coord>           : ");
    println!("  --version                     : show application version number({})", APPLICATION_VERSION);
}

#[derive(Debug)]
struct ConfigurationInformation {
    door_sections: usize,
    door_width: usize,
    gm_map_file: String,
    input_file: String,
    player_map_file: String,
    start_x: usize,
    start_y: usize,
}

impl ConfigurationInformation {
    fn new() -> Self {
        Self {
            door_sections: 1,
            door_width: 100,
            gm_map_file: "gm_map.svg".to_string(),
            input_file: "test_lab.xml".to_string(),
            player_map_file: "player_map.svg".to_string(),
            start_x: 0,
            start_y: 200,
        }
    }
    fn get_door_sections(&self) -> usize {
        self.door_sections
    }

    fn get_door_width(&self) -> usize {
        self.door_width
    }

    fn get_input_file_name<'a>(&'a self) -> &'a str {
        &self.input_file
    }

    fn get_gm_map_file_name<'a>(&'a self) -> &'a str {
        &self.gm_map_file
    }

    fn get_player_map_file_name<'a>(&'a self) -> &'a str {
        &self.player_map_file
    }

    fn get_start_x(&self) -> usize {
        self.start_x
    }

    fn get_start_y(&self) -> usize {
        self.start_y
    }

    fn set_door_sections(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.door_sections = filename.parse::<usize>().unwrap();
                if self.door_sections < 1 || self.door_sections > 2 {
                    println!("EEE --door-sections can only be 1 or 2")
                    // TODO fail here
                }
            }
            None => { println!("EEE the parameter was not there set_door_width()") }
        }
    }

    fn set_door_width(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.door_width = filename.parse::<usize>().unwrap();
            }
            None => { println!("EEE the parameter was not there set_door_width()") }
        }
    }

    fn set_gm_map_file(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.gm_map_file = filename.clone();
            }
            None => { println!("EEE the parameter was not there set_gm_map_file()") }
        }
    }

    fn set_player_map_file(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.player_map_file = filename.clone();
            }
            None => { println!("EEE the parameter was not there set_player_map_file()") }
        }
    }

    fn set_start_x(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.start_x = filename.parse::<usize>().unwrap();
            }
            None => { println!("EEE the parameter was not there set_start_x()") }
        }
    }

    fn set_start_y(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.start_y = filename.parse::<usize>().unwrap();
            }
            None => { println!("EEE the parameter was not there set_start_y()") }
        }
    }

    fn set_input_file(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.input_file = filename.clone();
            }
            None => { println!("EEE the parameter was not there set_input_file()") }
        }
    }
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

#[derive(Debug)]
pub struct Door {
    m_start_x: usize,
    m_start_y: usize,
    m_end_x: usize,
    m_end_y: usize,
}

fn read_map_definitions(xml_root: &Element, base_unit_in_svg: usize) -> (usize, usize) {
    let xml_map_definitions = xml_root.get_child("MapDefinitions", DEFAULT_NAME_SPACE).unwrap();
    let xml_map_width = xml_map_definitions.get_child("MapWidth", DEFAULT_NAME_SPACE).unwrap();
    let xml_map_height = xml_map_definitions.get_child("MapHeight", DEFAULT_NAME_SPACE).unwrap();
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
    child_element_name: &str
) -> Option<&'a minidom::Element> {
    for xml_child in xml_element.children() {
        if xml_child.name() == child_element_name {
            return Some(xml_child);
        }
    }
    None
}