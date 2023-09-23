

pub mod StateGenerator {
    use uuid::Uuid;
    use hex::encode;

    pub fn generate() -> String {
        let random_string = Uuid::new_v4();
        hex::encode(random_string.as_bytes())
    }
}