#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.iter().cloned().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result = self.scores.to_vec();
        result.sort();
        result.reverse();
        result.truncate(3);
        result
    }
}
