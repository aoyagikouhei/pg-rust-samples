use uuid::Uuid;

pub fn execute() {
    let uuid = Uuid::now_v7();
    let time = common::utils::get_utc(&uuid);
    println!("uuid: {:?}", uuid);
    println!("time: {:?}", time);
}