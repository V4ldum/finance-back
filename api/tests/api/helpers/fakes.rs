use fake::{Fake, faker::lorem::en::Sentence};

pub fn name() -> String {
    Sentence(1..3).fake()
}

pub fn fake_id() -> i64 {
    (1..1000).fake()
}

pub fn possessed() -> i64 {
    (1..1000).fake()
}

pub fn unit_value() -> i64 {
    (1..1000).fake()
}

pub fn unit_weight() -> i64 {
    (1..1000).fake()
}

pub fn composition() -> String {
    ["GOLD", "SILVER"][(0..2).fake::<usize>()].to_string()
}

pub fn purity() -> i64 {
    (1..=9999).fake()
}

pub fn gold_price() -> f64 {
    (2000.0..5000.0).fake()
}

pub fn silver_price() -> f64 {
    (50.0..100.0).fake()
}

pub fn sp_price() -> f64 {
    (5000.0..9000.0).fake()
}
