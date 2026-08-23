use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
}
