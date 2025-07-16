use nanoid::nanoid;

pub fn generate_id() -> String {
    nanoid!(10)
}
