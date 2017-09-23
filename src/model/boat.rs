pub struct Boat {
    boat_type: String, //should be enum?
    length: f64
}

impl Boat {
    fn get_type(&self) -> &str {
        &self.boat_type[..]
    }

    fn get_length(&self) -> &f64 {
        &self.length
    }
}