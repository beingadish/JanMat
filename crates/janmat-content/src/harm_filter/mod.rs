#[derive(Debug, Clone, Copy)]
pub struct HarmScore(pub u8);

#[derive(Default)]
pub struct HarmFilter;

impl HarmFilter {
    pub fn score(&self, _text: &str) -> HarmScore {
        HarmScore(0)
    }
}
