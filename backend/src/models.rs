use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================================
// TENANT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub domain: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// USER
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub department: Department,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Manager,
    Employee,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Department {
    Hr,
    Finance,
    It,
    Operations,
    Sales,
}

// ============================================================================
// WORKFLOW CONFIG
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowConfig {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub department: Department,
    pub workflow_type: String,
    pub name: String,
    pub description: Option<String>,
    pub approval_chain: serde_json::Value,
    pub required_approvals: i32,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// WORKFLOW
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workflow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub config_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: WorkflowStatus,
    pub current_step: i32,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
    Completed,
}

// ============================================================================
// WORKFLOW APPROVAL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowApproval {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub workflow_id: Uuid,
    pub approver_id: Uuid,
    pub step_number: i32,
    pub status: ApprovalStatus,
    pub comments: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

// ============================================================================
// AUDIT LOG
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// DTOs (Data Transfer Objects)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub department: Department,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            department: user.department,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowRequest {
    pub config_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ApproveWorkflowRequest {
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowConfigRequest {
    pub department: Department,
    pub workflow_type: String,
    pub name: String,
    pub description: Option<String>,
    pub approval_chain: serde_json::Value,
    pub required_approvals: i32,
}

// ============================================================================
// JWT Claims
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,        // user_id
    pub tenant_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub exp: usize,
}
