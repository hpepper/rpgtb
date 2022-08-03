use std::fs::File;
use std::io::Write;

fn lead_in(file_handle: &mut File, map_width: usize, map_height: usize) {
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

fn lead_out(file_handle: &mut File) {
    file_handle.write(b"</svg>\n").expect("file write error");
}

fn render_room(file_handle: &mut File, room: crate::Room) {
    let line_thickness: usize = 2;
    file_handle.write(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke=\"black\" fill=\"transparent\" stroke-width=\"{}\"/>\n", room.m_start_x, room.m_start_y, room.m_width, room.m_height, line_thickness).as_bytes()).expect("file write error");
}

// TODO
pub fn render(map_width: usize, map_height: usize, room: crate::Room) {
    println!("DDD render");
    let mut file_handle = File::create("test.svg").expect("Error encountered while creating file!");
    lead_in(&mut file_handle, map_width, map_height);
    render_room(&mut file_handle, room);
    lead_out(&mut file_handle);
}
