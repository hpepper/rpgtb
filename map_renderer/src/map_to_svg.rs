use std::fs::File;
use std::io::Write;

// TODO make these references commonly
const DOOR_HINGE_LEFT: bool = true;
const DOOR_HINGE_RIGHT: bool = false;

// TODO remove 'pub' when I don't need it for debugging the door gen.
pub fn lead_in(file_handle: &mut File, map_width: usize, map_height: usize) {
    file_handle.write(b"<svg version=\"1.1\"\n").expect("file write error");
    file_handle
        .write(&format!(" width=\"{}\" height=\"{}\"\n", map_width, map_height).as_bytes())
        .expect("file write error");
    file_handle.write(b" xmlns=\"http://www.w3.org/2000/svg\">\n").expect("file write error");
    file_handle.write(b"\n").expect("file write error");
}

pub fn lead_out(file_handle: &mut File) {
    file_handle.write(b"</svg>\n").expect("file write error");
}

pub fn render_door(
    file_handle: &mut File,
    door: &crate::Door,
    room_base_x: &usize,
    room_base_y: &usize,
    left_hinged: bool
) {
    println!("DDD Door: {:?}", door);
    if door.m_start_y == door.m_end_y {
        render_door_horizontal(file_handle, &door, &room_base_x, &room_base_y, left_hinged);
    } else {
        render_door_vertical(file_handle, &door, &room_base_x, &room_base_y, left_hinged);
    }
}


pub fn render_door_horizontal(
    file_handle: &mut File,
    door: &crate::Door,
    room_base_x: &usize,
    room_base_y: &usize,
    left_hinged: bool
) {
    println!("DDD render_door_horizontal() Door: {:?}", door);
    // If the length is two, then splite the door and call itself twice
    if door.m_number_sections == 2 {
        println!("DDD two sections");
        let half_door_length = (door.m_end_x - door.m_start_x) / 2;
        let left_door = crate::Door {
            m_start_x: door.m_start_x,
            m_start_y: door.m_start_y,
            m_end_x: door.m_start_x + half_door_length,
            m_end_y: door.m_end_y,
            m_number_sections: 1,
        };
        render_door(file_handle, &left_door, &room_base_x, &room_base_y, DOOR_HINGE_LEFT);

        let right_door = crate::Door {
            m_start_x: door.m_start_x + half_door_length,
            m_start_y: door.m_start_y,
            m_end_x: door.m_end_x,
            m_end_y: door.m_end_y,
            m_number_sections: 1,
        };
        render_door(file_handle, &right_door, &room_base_x, &room_base_y, DOOR_HINGE_RIGHT);
    } else {
        // TODO move this block to a sub function? render_single_door_section
        // TODO Handle er vertical door.
        let line_thickness: usize = 1;

        // triangle_a - the hinge point of the door
        let (triangle_a_x, triangle_a_y) = if left_hinged {
            (0.0, 0.0)
        } else {
            (
                (door.m_end_x as f32) - (door.m_start_x as f32),
                (door.m_end_y as f32) - (door.m_start_y as f32),
            )
        };

        // triangle_b - the the point of the door away from the hinge(handle point)
        let (triangle_b_x, triangle_b_y) = if left_hinged {
            (
                (door.m_end_x as f32) - (door.m_start_x as f32),
                (door.m_end_y as f32) - (door.m_start_y as f32),
            )
        } else {
            (0.0, 0.0)
        };

        let pi = 3.14;
        let degrees: f32 = 25.0;
        let radians = (degrees * pi) / 180.0;

        // triangle_c - the open point of the door
        let (triangle_c_x, triangle_c_y) = if left_hinged {
            (radians.cos() * triangle_b_x, radians.sin() * triangle_b_x)
        } else {
            (triangle_a_x - radians.cos() * triangle_a_x, radians.sin() * triangle_a_x)
        };

        /*
      TODO kept here for when the doors are going to be done vertically.
    println!("DDD render_door(x, {:#?}, {}, {}) ", door, room_base_x, room_base_y);
    println!("DDD radians: {}, cos: {}, sin: {}", radians, radians.cos(), radians.sin());
   */
        println!(
            "DDD triangle(A: {},{} B: {},{} C: {},{}) ",
            triangle_a_x,
            triangle_a_y,
            triangle_b_x,
            triangle_b_y,
            triangle_c_x,
            triangle_c_y
        );

        let start_x: f32 = (*room_base_x as f32) + (door.m_start_x as f32);
        let start_y: f32 = (*room_base_y as f32) + (door.m_start_y as f32);

        file_handle
            .write(
                &format!(
                    "<path d=\"M {} {}",
                    start_x + triangle_a_x,
                    start_y + triangle_a_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle
            .write(
                &format!(
                    " l {} {}",
                    triangle_c_x - triangle_a_x,
                    -triangle_c_y + triangle_a_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle.write(&format!(" \"").as_bytes()).expect("file write error");
        file_handle
            .write(&format!(" stroke=\"black\" fill=\"transparent\"").as_bytes())
            .expect("file write error");
        file_handle
            .write(&format!(" stroke-width=\"{}\"", line_thickness).as_bytes())
            .expect("file write error");
        file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");

        let curvepoint_y = triangle_c_y / 2.0;

        file_handle
            .write(
                &format!(
                    "<path d=\"M {} {}",
                    start_x + triangle_b_x,
                    start_y + triangle_b_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle
            .write(&format!(" q {} {},", 0, -curvepoint_y).as_bytes())
            .expect("file write error");
        file_handle
            .write(&format!(" {} {}\"", triangle_c_x - triangle_b_x, -triangle_c_y).as_bytes())
            .expect("file write error");

        file_handle
            .write(&format!(" stroke=\"black\" fill=\"none\" stroke-width=\"1\"").as_bytes())
            .expect("file write error");
        file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");
    }
}


pub fn render_door_vertical(
    file_handle: &mut File,
    door: &crate::Door,
    room_base_x: &usize,
    room_base_y: &usize,
    left_hinged: bool
) {
    println!("DDD render_door_vertical() Door: {:?}", door);
    // If the length is two, then splite the door and call itself twice
    if door.m_number_sections == 2 {
        println!("DDD two sections");
        let half_door_length = (door.m_end_y - door.m_start_y) / 2;
        let left_door = crate::Door {
            m_start_x: door.m_start_x,
            m_start_y: door.m_start_y,
            m_end_x: door.m_end_x,
            m_end_y: door.m_start_y + half_door_length,
            m_number_sections: 1,
        };
        render_door(file_handle, &left_door, &room_base_x, &room_base_y, DOOR_HINGE_LEFT);

        let right_door = crate::Door {
            m_start_x: door.m_start_x,
            m_start_y: door.m_start_y + half_door_length,
            m_end_x: door.m_end_x,
            m_end_y: door.m_end_y,
            m_number_sections: 1,
        };
        render_door(file_handle, &right_door, &room_base_x, &room_base_y, DOOR_HINGE_RIGHT);
    } else {
        // TODO move this block to a sub function? render_single_door_section
        // TODO Handle er vertical door.
        let line_thickness: usize = 1;

        // triangle_a - the hinge point of the door
        let (triangle_a_x, triangle_a_y) = if left_hinged {
            (0.0, 0.0)
        } else {
            (
                (door.m_end_x as f32) - (door.m_start_x as f32),
                (door.m_end_y as f32) - (door.m_start_y as f32),
            )
        };

        // triangle_b - the the point of the door away from the hinge(handle point)
        let (triangle_b_x, triangle_b_y) = if left_hinged {
            (
                (door.m_end_x as f32) - (door.m_start_x as f32),
                (door.m_end_y as f32) - (door.m_start_y as f32),
            )
        } else {
            (0.0, 0.0)
        };

        let pi = 3.14;
        let degrees: f32 = 25.0;
        let radians = (degrees * pi) / 180.0;

        // triangle_c - the open point of the door
        let (triangle_c_y, triangle_c_x) = if left_hinged {
            (radians.cos() * triangle_b_y, radians.sin() * triangle_b_y)
        } else {
            (triangle_a_y - radians.cos() * triangle_a_y, radians.sin() * triangle_a_y)
        };

        /*
      TODO kept here for when the doors are going to be done vertically.
    println!("DDD render_door(x, {:#?}, {}, {}) ", door, room_base_x, room_base_y);
    println!("DDD radians: {}, cos: {}, sin: {}", radians, radians.cos(), radians.sin());
   */
        println!(
            "DDD triangle(A: {},{} B: {},{} C: {},{}) ",
            triangle_a_x,
            triangle_a_y,
            triangle_b_x,
            triangle_b_y,
            triangle_c_x,
            triangle_c_y
        );

        let start_x: f32 = (*room_base_x as f32) + (door.m_start_x as f32);
        let start_y: f32 = (*room_base_y as f32) + (door.m_start_y as f32);

        file_handle
            .write(
                &format!(
                    "<path d=\"M {} {}",
                    start_x + triangle_a_x,
                    start_y + triangle_a_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle
            .write(
                &format!(
                    " l {} {}",
                    triangle_c_x - triangle_a_x,
                    -triangle_c_y + triangle_a_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle.write(&format!(" \"").as_bytes()).expect("file write error");
        file_handle
            .write(&format!(" stroke=\"black\" fill=\"transparent\"").as_bytes())
            .expect("file write error");
        file_handle
            .write(&format!(" stroke-width=\"{}\"", line_thickness).as_bytes())
            .expect("file write error");
        file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");

        let curvepoint_x = triangle_c_x / 2.0;

        file_handle
            .write(
                &format!(
                    "<path d=\"M {} {}",
                    start_x + triangle_b_x,
                    start_y - triangle_b_y
                ).as_bytes()
            )
            .expect("file write error");
        file_handle
            .write(&format!(" q {} {},", curvepoint_x, 0).as_bytes())
            .expect("file write error");
        file_handle
            .write(&format!(" {} {}\"", triangle_c_x,  triangle_b_y - triangle_c_y).as_bytes())
            .expect("file write error");

        file_handle
            .write(&format!(" stroke=\"black\" fill=\"none\" stroke-width=\"1\"").as_bytes())
            .expect("file write error");
        file_handle.write(&format!("/>\n").as_bytes()).expect("file write error");
    }
}

fn render_room(file_handle: &mut File, room: &crate::Room, gms_map: bool) {
    let line_thickness: usize = 2;
    file_handle
        .write(
            &format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke=\"black\" fill=\"transparent\" stroke-width=\"{}\"/>\n",
                room.m_start_x,
                room.m_start_y,
                room.m_width,
                room.m_height,
                line_thickness
            ).as_bytes()
        )
        .expect("file write error");
    if gms_map {
        // TODO fix it so the text is always in the room, no matter what direction the rooom is drawn(left-to-right, or right-to-left)
        file_handle
            .write(
                &format!(
                    "  <text x=\"{}\" y=\"{}\" fill=\"red\">{}</text>\n",
                    room.m_start_x + 2,
                    room.m_start_y + 12,
                    room.gm_note
                ).as_bytes()
            )
            .expect("file write error");
    }
    //
    for door in room.door_list.iter() {
        render_door(file_handle, door, &room.m_start_x, &room.m_start_y, true);
    }
}

fn render_to_file(
    file_name: &str,
    map_width: usize,
    map_height: usize,
    room_list: &Vec<crate::Room>,
    gms_map: bool
) {
    let mut file_handle = File::create(file_name).expect("Error encountered while creating file!");
    lead_in(&mut file_handle, map_width, map_height);
    for room in room_list.iter() {
        render_room(&mut file_handle, room, gms_map);
    }
    lead_out(&mut file_handle);
}

pub fn render(
    player_map_name: &str,
    gm_map_name: &str,
    map_width: usize,
    map_height: usize,
    room_list: Vec<crate::Room>
) {
    println!("III render maps");
    println!("III   player map: {}", player_map_name);
    println!("III   gm map    : {}", gm_map_name);
    render_to_file(player_map_name, map_width, map_height, &room_list, false);
    render_to_file(gm_map_name, map_width, map_height, &room_list, true);
}