use std::fs::File;
use std::io::Write;

// TODO remove 'pub' when I don't need it for debugging the door gen.
pub fn lead_in(file_handle: &mut File, map_width: usize, map_height: usize) {
    file_handle
        .write(b"<svg version=\"1.1\"\n")
        .expect("file write error");
    file_handle
        .write(&format!(" width=\"{}\" height=\"{}\"\n", map_width, map_height).as_bytes())
        .expect("file write error");
    file_handle
        .write(b" xmlns=\"http://www.w3.org/2000/svg\">\n")
        .expect("file write error");
    file_handle.write(b"\n").expect("file write error");
}

pub fn lead_out(file_handle: &mut File) {
    file_handle.write(b"</svg>\n").expect("file write error");
}


pub fn render_door(file_handle: &mut File, door: &crate::Door, room_base_x: usize, room_base_y: usize) {
    let line_thickness: usize = 1;

    let delta_x = door.m_end_x - door.m_start_x;
    let delta_y = door.m_end_y - door.m_start_y;
    let float_delta_x: f32 = delta_x as f32;
    let float_delta_y: f32 = delta_y as f32;

    println!("DDD render_door(x, {:#?}, {}, {}) ", door, room_base_x, room_base_y);

    // These three line is my convoluted way to get the sqrt of the a² + b²
    //  I know not how to to the sqrt directly
    // let length = delta_x.pow(2) + delta_y.pow(2);
    // let float_length: f32 = length as f32;
    // TODO where should I use this?
    // let door_width:f32 = float_length.sqrt();

    let pi = 3.14;
    let degrees: f32 = 25.0;
    let radians = (degrees * pi) / 180.0;

    let new_x = float_delta_x * radians.cos() - float_delta_y * radians.sin();
    let new_y = float_delta_x * radians.sin() + float_delta_y * radians.cos();

    let start_x = room_base_x + door.m_start_x;
    let start_y = room_base_y + door.m_start_y;

    let end_x_open_door = start_x as f32 + new_x;
    let end_y_open_door = start_y as f32 - new_y;
    
    file_handle.write(&format!("<line x1=\"{}\"", start_x).as_bytes()).expect("file write error");
    file_handle.write(&format!(" y1=\"{}\"", start_y).as_bytes()).expect("file write error");
    file_handle.write(&format!(" x2=\"{}\"", start_x as f32 + new_x).as_bytes()).expect("file write error");
    file_handle.write(&format!(" y2=\"{}\"", start_y as f32 - new_y).as_bytes()).expect("file write error");
    file_handle.write(&format!(" stroke=\"black\" fill=\"transparent\"").as_bytes()).expect("file write error");
    file_handle.write(&format!(" stroke-width=\"{}\"", line_thickness).as_bytes()).expect("file write error");
    file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");

    // TODO do the circle thingy indicating the door openeing, using the M x y q xxxxx (quadratic bezier)
    // https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
    let curvepoint_x: f32 = (door.m_end_x as f32 - end_x_open_door) / 2.0;
    let curvepoint_y: f32 = door.m_end_x as f32 - end_x_open_door;
    let delta_end_curve_x: f32 = door.m_end_x as f32 - end_x_open_door;
    let delta_end_curve_y: f32 = door.m_end_y as f32 - end_y_open_door;
    file_handle.write(&format!("<path d=\"M {} {}", end_x_open_door, end_y_open_door).as_bytes()).expect("file write error");
    file_handle.write(&format!(" q {} {},", curvepoint_x, curvepoint_y).as_bytes()).expect("file write error");
    file_handle.write(&format!(" {} {}\"", delta_end_curve_x, delta_end_curve_y).as_bytes()).expect("file write error");
    file_handle.write(&format!(" stroke=\"black\" fill=\"none\" stroke-width=\"1\"").as_bytes()).expect("file write error");
    file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");
}


fn render_room(file_handle: &mut File, room: &crate::Room, gms_map: bool) {
    let line_thickness: usize = 2;
    file_handle.write(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke=\"black\" fill=\"transparent\" stroke-width=\"{}\"/>\n", room.m_start_x, room.m_start_y, room.m_width, room.m_height, line_thickness).as_bytes()).expect("file write error");
    if gms_map {
        // TODO fix it so the text is always in the room, no matter what direction the rooom is drawn(left-to-right, or right-to-left)
        file_handle.write(&format!("  <text x=\"{}\" y=\"{}\" fill=\"red\">{}</text>\n", room.m_start_x+2, room.m_start_y+12, room.gm_note).as_bytes()).expect("file write error");
    }
    // 
    for door in room.door_list.iter() {
        render_door(file_handle, door, room.m_start_x, room.m_start_y);
    }
}

fn render_to_file(file_name: &str, map_width: usize, map_height: usize, room_list: &Vec<crate::Room>, gms_map: bool) {
    let mut file_handle = File::create(file_name).expect("Error encountered while creating file!");
    lead_in(&mut file_handle, map_width, map_height);
    for room in room_list.iter() {
        render_room(&mut file_handle, room, gms_map);
    }
    lead_out(&mut file_handle);
}

pub fn render(player_map_name: &str, gm_map_name: &str, map_width: usize, map_height: usize, room_list: Vec<crate::Room>) {
    println!("III render maps");
    println!("III   player map: {}", player_map_name);
    println!("III   gm map    : {}", gm_map_name);
    render_to_file(player_map_name, map_width,map_height,  &room_list,false);
    render_to_file(gm_map_name, map_width,map_height,  &room_list,true);
}