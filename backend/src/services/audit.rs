use sqlx::PgPool;
use uuid::Uuid;

/// Audit service for immutable event logging
pub struct AuditService {
    db: PgPool,
}

impl AuditService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Log a workflow event
    pub async fn log_workflow_event(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        workflow_id: Uuid,
        action: &str,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs 
            (tenant_id, user_id, entity_type, entity_id, action, old_values, new_values)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind("WORKFLOW")
        .bind(workflow_id)
        .bind(action)
        .bind(old_values)
        .bind(new_values)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// Get audit trail for a specific entity
    pub async fn get_audit_trail(
        &self,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Vec<crate::models::AuditLog>, anyhow::Error> {
        let logs = sqlx::query_as(
            r#"
            SELECT * FROM audit_logs 
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&self.db)
        .await?;

        Ok(logs)
    }
}
