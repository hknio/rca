pub struct Information {
    pub remote: String,
    pub local: String,
    pub sloc: usize,
}

pub struct Quality {}

pub struct Security {}

pub struct Report {
    pub information: Information,
    pub quality: Quality,
    pub security: Security,
}
