use crate::error::AppError;

pub(super) fn interpolate(from: &[f64], to: &[f64], factor: f64) -> Result<Vec<f64>, AppError> {
    if from.len() != to.len() {
        return Err(AppError::InvalidRequestConfig(format!(
            "twitter interpolation requires arrays of the same length, got {} and {}",
            from.len(),
            to.len()
        )));
    }

    Ok(from
        .iter()
        .zip(to.iter())
        .map(|(from, to)| from * (1.0 - factor) + to * factor)
        .collect())
}
