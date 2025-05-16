#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut output = Vec::with_capacity(scores.len());

        for score in scores {
            output.push(*score);
        }

        Self(output)
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut output = self.0.clone();

        output.sort_unstable();
        output.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut dummy = self.0.clone();
        let mut output = vec![];

        dummy.sort_unstable();
        for (_, score) in dummy
            .iter()
            .enumerate()
            .skip_while(|v| v.0 != dummy.len().saturating_sub(3))
        {
            output.push(*score);
        }

        output.into_iter().rev().collect::<Vec<_>>()
    }
}
