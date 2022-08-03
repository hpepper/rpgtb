mod map;

use minidom::Element;
use std::fs;

const DEFAULT_NAME_SPACE: &str = "minidom";

// TODO move this to its own file(module?)
pub struct Room {
    m_start_x: usize,
    m_start_y: usize,
    m_width: usize,
    m_height: usize,
    // TODO door list
    // TODO window list
}
/*
fn generate_map_struct_from_xml_file() {
}
 */

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

fn load_room(xml_root: &Element, base_unit_in_svg: usize) -> Room {
    let xml_room = xml_root.get_child("Room", DEFAULT_NAME_SPACE).unwrap();

    let xpos_str = xml_room.attr("xpos").unwrap();
    let ypos_str = xml_room.attr("ypos").unwrap();
    let width_str = xml_room.attr("width").unwrap();
    let height_str = xml_room.attr("height").unwrap();
    let xpos:usize = xpos_str.parse().unwrap();
    let ypos:usize = ypos_str.parse().unwrap();
    let width:usize = width_str.parse().unwrap();
    let height:usize = height_str.parse().unwrap();

    Room {
        m_start_x: xpos * base_unit_in_svg,
        m_start_y: ypos * base_unit_in_svg,
        m_width: width * base_unit_in_svg,
        m_height: height * base_unit_in_svg,
    }
}

fn main() {
    let xml_string = fs::read_to_string("test_lab.xml").expect("File not found");

    let xml_root: Element = xml_string.parse().unwrap();
    let version = xml_root.attr("version").unwrap();
    let xml_base_unit_svg = xml_root
        .get_child("BaseUnitInSvg", DEFAULT_NAME_SPACE)
        .unwrap();
    let baseunit_text = xml_base_unit_svg.text();
    let base_unit_in_svg: usize = baseunit_text.parse().unwrap();

    let (map_width, map_height) = read_map_definitions(&xml_root, base_unit_in_svg);

    let a_room = load_room(&xml_root, base_unit_in_svg);

    // TODO RESUME: create the render module and do the lead in and lead out of that.
    map::render(map_width, map_height, a_room);
}
