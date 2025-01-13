use super::*;

/// Rerank lists of items.
///
/// Given multiple lists of items, this struct can rerank them using the
/// Reciprocal Rank Fusion (RRF) algorithm. The RRF algorithm assigns a score
/// to each item based on its rank in the lists and then returns the top-k
/// items with the highest scores.
pub struct Reranker<T> {
    lists: Vec<Vec<T>>,
}

impl<T> Reranker<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates a new instance of the Reranker.
    pub fn new(lists: Vec<Vec<T>>) -> Self {
        Self { lists }
    }

    /// Reranks the items using the Reciprocal Rank Fusion algorithm.
    /// - constant: Number to add to the rank of each item.
    /// - k: Number of items to return.
    pub fn rrf(&self, constant: usize, k: u8) -> Vec<T> {
        let mut scores: HashMap<T, f32> = HashMap::new();

        for ranking in self.lists.iter() {
            for (rank, item) in ranking.iter().enumerate() {
                let score = 1.0 / ((rank + 1) + constant) as f32;
                *scores.entry(item.clone()).or_insert(0.0) += score;
            }
        }

        let mut items: Vec<(T, f32)> = scores.into_iter().collect();
        items.sort_by(|(_, a), (_, b)| {
            b.partial_cmp(a).unwrap_or(Ordering::Equal)
        });

        items
            .iter()
            .take(k as usize)
            .map(|(item, _)| item.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf() {
        let reranker = setup();
        let ranked = reranker.rrf(60, 3);
        assert_eq!(ranked, vec![4, 1, 3]);
    }

    fn setup() -> Reranker<u8> {
        Reranker::new(vec![
            vec![1, 2, 3, 4],
            vec![1, 3, 4, 5],
            vec![4, 5, 6, 7],
        ])
    }
}
