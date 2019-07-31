#![allow(unused_variables)]
#![doc(hidden)]

extern crate cemu_smm;

use protobuf::parse_from_bytes;
use protobuf::Message;
use std::fs::read;

use cemu_smm::course::Course;
use cemu_smm::course2::Course2;
use cemu_smm::proto::SMMCourse::SMMCourse;

fn main() {
    let file = read("tests/assets/courses/course001/course").unwrap();
    let course: SMMCourse = parse_from_bytes(file.as_slice()).unwrap();
    println!("{:?}", course.sounds);

    let mut out: Vec<u8> = vec![];
    course.write_to_vec(&mut out).unwrap();
    let new_course: SMMCourse = parse_from_bytes(out.as_slice()).unwrap();
    println!("{:?}", new_course.sounds);

    let course = Course::from_wii_u_files(
        &read("tests/assets/courses/course001/course_data.cdt").unwrap(),
        &read("tests/assets/courses/course001/course_data_sub.cdt").unwrap(),
        &read("tests/assets/courses/course001/thumbnail0.tnl").unwrap(),
        &read("tests/assets/courses/course001/thumbnail1.tnl").unwrap(),
    );
    // dbg!(&course);

    let mut file = read("tests/assets/saves/smm2/course_data_120.bcd").unwrap();
    let course = Course2::from_switch_file(&mut file[..]).unwrap();
    dbg!(&course);
    dbg!(&course.get_course_ref().get_header().game_style);
    dbg!(&course.get_course_ref().get_course_area().auto_scroll);
    dbg!(&course.get_course_ref().get_course_sub_area().auto_scroll);

    // let decrypted = cemu_smm::course2::Course2::decrypt(file.to_vec());
    // dbg!(decrypted[0xF1] as char, decrypted[0xF2] as char);
}
