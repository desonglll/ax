use actix_web::web;

use crate::errors::AxError;
use crate::services::features::get_user_features;
use crate::services::ml_predict::predict;
use crate::state::AppState;

/// Recommend posts for a user.
///
/// This function retrieves behavioral features for the specified USER_ID,
/// and queries the machine learning prediction service to return a list of post IDs.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `user_id`: The identifier of the user.
///
/// # Returns
///
/// A vector of recommended post IDs on success, or an [`AxError`] on failure.
pub async fn recommend_posts(
    app_state: web::Data<AppState>,
    user_id: i32,
) -> Result<Vec<uuid::Uuid>, AxError> {
    // Construct model features from user stats.
    let model_input = get_user_features(&app_state.db, user_id).await?;

    // Invoke the prediction service.
    match predict(model_input).await {
        Ok(recommended_ids) => Ok(recommended_ids),
        Err(e) => {
            crate::infra::log::Log::warning(format!(
                "Prediction service failed (error: {:?}). Falling back to database heuristic.",
                e
            ));
            // Call the database fallback query
            crate::dbaccess::post::get_trending_posts_fallback_db(&app_state.db).await
        }
    }
}
