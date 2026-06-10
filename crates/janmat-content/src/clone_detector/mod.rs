pub struct CloneMatch {
    pub candidate_id: String,
    pub score: f32,
}

#[derive(Default)]
pub struct CloneDetector;

impl CloneDetector {
    pub fn scan(&self, _text: &str) -> Vec<CloneMatch> {
        Vec::new()
    }
}
