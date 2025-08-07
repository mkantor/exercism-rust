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
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let top_scores =
            self.scores
                .iter()
                .fold([None, None, None], |[first, second, third], &current| {
                    if Some(current) >= first {
                        [Some(current), first, second]
                    } else if Some(current) >= second {
                        [first, Some(current), second]
                    } else if Some(current) >= third {
                        [first, second, Some(current)]
                    } else {
                        [first, second, third]
                    }
                });

        top_scores.into_iter().flatten().collect()
    }
}
