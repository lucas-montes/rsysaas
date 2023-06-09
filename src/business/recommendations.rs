use crate::data::{errors::CRUDError, interface::get_model_items};
use crate::web::requests::RecommendationQueryRequest;
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::interface::CustomerInterface;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recommendation {
    prod_id: u32,
    rank: f32,
    path: String,
}

impl Recommendation {
    fn new(prod_id: u32, rank: f32, domain: Arc<String>) -> Self {
        Recommendation {
            prod_id,
            rank,
            path: Self::generate_path(domain, prod_id),
        }
    }

    pub async fn generate_recommendations(
        customer: &CustomerInterface,
        request: &RecommendationQueryRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        let (item, references) = match Self::get_items(&customer, request).await {
            Ok((item, references)) => (item, references),
            Err(e) => return Err(e),
        };
        Ok(Self::calculate_recommendations(
            item,
            references,
            request.num_recs.unwrap_or(5),
            customer.domain.clone(),
        )
        .await)
    }

    async fn get_items(
        customer: &CustomerInterface,
        request: &RecommendationQueryRequest,
    ) -> Result<(Item, Vec<Item>), CRUDError> {
        Ok(get_model_items(request.prod_id.unwrap(), request.entity.clone()).await)
    }

    async fn calculate_recommendations(
        item: Item,
        references: Vec<Item>,
        num_recs: u8,
        domain: Arc<String>,
    ) -> Vec<Recommendation> {
        let knn = KNN::new(item, references, num_recs);
        knn.result(SimilarityAlgos::Cosine)
            .into_iter()
            .map(|item| Recommendation::new(item.id, item.result, domain.clone()))
            .collect()
    }

    fn generate_path(domain: Arc<String>, prod_id: u32) -> String {
        format!("my/path/{domain}/{prod_id}/")
    }
}
