use crate::models::DistanceMetric;

/// Calculate similarity between two vectors based on the distance metric
pub fn calculate_similarity(vec1: &[f32], vec2: &[f32], metric: DistanceMetric) -> f32 {
    match metric {
        DistanceMetric::Cosine => cosine_similarity(vec1, vec2),
        DistanceMetric::Euclidean => -euclidean_distance(vec1, vec2), // Negative for sorting
        DistanceMetric::DotProduct => dot_product(vec1, vec2),
    }
}

/// Cosine similarity: measures the cosine of the angle between vectors
pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return 0.0;
    }

    let dot = dot_product(vec1, vec2);
    let norm1 = magnitude(vec1);
    let norm2 = magnitude(vec2);

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    dot / (norm1 * norm2)
}

/// Euclidean distance: straight-line distance between vectors
pub fn euclidean_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return f32::MAX;
    }

    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Dot product: sum of element-wise multiplication
pub fn dot_product(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return 0.0;
    }

    vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum()
}

/// Calculate magnitude (norm) of a vector
pub fn magnitude(vec: &[f32]) -> f32 {
    vec.iter().map(|x| x * x).sum::<f32>().sqrt()
}

/// Normalize a vector to unit length
pub fn normalize(vec: &[f32]) -> Vec<f32> {
    let mag = magnitude(vec);
    if mag == 0.0 {
        return vec.to_vec();
    }
    vec.iter().map(|x| x / mag).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&vec1, &vec2) - 1.0).abs() < 1e-6);

        let vec3 = vec![1.0, 0.0, 0.0];
        let vec4 = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&vec3, &vec4)).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        assert!(euclidean_distance(&vec1, &vec2).abs() < 1e-6);

        let vec3 = vec![0.0, 0.0, 0.0];
        let vec4 = vec![1.0, 1.0, 1.0];
        assert!((euclidean_distance(&vec3, &vec4) - 1.732).abs() < 0.01);
    }

    #[test]
    fn test_normalize() {
        let vec = vec![3.0, 4.0];
        let normalized = normalize(&vec);
        assert!((magnitude(&normalized) - 1.0).abs() < 1e-6);
    }
}

