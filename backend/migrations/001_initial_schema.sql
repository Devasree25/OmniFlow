-- OmniFlow Platform - Initial Database Schema
-- Multi-tenant SaaS with Row-Level Security

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- TENANTS TABLE
-- ============================================================================
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    domain VARCHAR(255) UNIQUE NOT NULL,
    settings JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_tenants_domain ON tenants(domain);
CREATE INDEX idx_tenants_active ON tenants(is_active);

-- ============================================================================
-- USERS TABLE
-- ============================================================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL CHECK (role IN ('SUPER_ADMIN', 'ADMIN', 'MANAGER', 'EMPLOYEE')),
    department VARCHAR(100) NOT NULL CHECK (department IN ('HR', 'FINANCE', 'IT', 'OPERATIONS', 'SALES')),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, email)
);

CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_department ON users(department);

-- ============================================================================
-- WORKFLOW CONFIGURATIONS TABLE
-- ============================================================================
CREATE TABLE workflow_configs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department VARCHAR(100) NOT NULL,
    workflow_type VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    approval_chain JSONB NOT NULL DEFAULT '[]',
    -- Example: [{"role": "MANAGER", "required": true}, {"role": "ADMIN", "required": false}]
    required_approvals INTEGER DEFAULT 1,
    settings JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, department, workflow_type)
);

CREATE INDEX idx_workflow_configs_tenant ON workflow_configs(tenant_id);
CREATE INDEX idx_workflow_configs_dept ON workflow_configs(department);

-- ============================================================================
-- WORKFLOWS TABLE (Instances)
-- ============================================================================
CREATE TABLE workflows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    config_id UUID NOT NULL REFERENCES workflow_configs(id),
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'IN_REVIEW', 'APPROVED', 'REJECTED', 'COMPLETED')),
    current_step INTEGER DEFAULT 0,
    data JSONB DEFAULT '{}',
    -- Stores workflow-specific data like leave dates, expense amounts, etc.
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_workflows_tenant ON workflows(tenant_id);
CREATE INDEX idx_workflows_status ON workflows(status);
CREATE INDEX idx_workflows_created_by ON workflows(created_by);
CREATE INDEX idx_workflows_config ON workflows(config_id);

-- ============================================================================
-- WORKFLOW APPROVALS TABLE
-- ============================================================================
CREATE TABLE workflow_approvals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    approver_id UUID NOT NULL REFERENCES users(id),
    step_number INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'APPROVED', 'REJECTED')),
    comments TEXT,
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_approvals_workflow ON workflow_approvals(workflow_id);
CREATE INDEX idx_approvals_approver ON workflow_approvals(approver_id);
CREATE INDEX idx_approvals_status ON workflow_approvals(status);

-- ============================================================================
-- AUDIT LOGS TABLE (Immutable)
-- ============================================================================
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(100) NOT NULL,
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_audit_tenant ON audit_logs(tenant_id);
CREATE INDEX idx_audit_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_created ON audit_logs(created_at);

-- ============================================================================
-- ROW-LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on all tenant-scoped tables
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_configs ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflows ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;

-- Create policies for tenant isolation
-- Users can only see data from their own tenant

CREATE POLICY tenant_isolation_users ON users
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_workflow_configs ON workflow_configs
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_workflows ON workflows
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_workflow_approvals ON workflow_approvals
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_audit_logs ON audit_logs
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- ============================================================================
-- TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tenants_updated_at BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workflow_configs_updated_at BEFORE UPDATE ON workflow_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workflows_updated_at BEFORE UPDATE ON workflows
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SEED DATA FOR DEMO
-- ============================================================================

-- Insert demo tenants
INSERT INTO tenants (id, name, domain, settings) VALUES
    ('11111111-1111-1111-1111-111111111111', 'Acme Corporation', 'acme.com', '{"max_users": 100, "features": ["hr", "finance"]}'),
    ('22222222-2222-2222-2222-222222222222', 'TechCorp Industries', 'techcorp.com', '{"max_users": 50, "features": ["hr", "it"]}');

-- Insert demo users (password: password123)
-- Password hash generated with bcrypt cost 12
INSERT INTO users (id, tenant_id, email, password_hash, first_name, last_name, role, department) VALUES
    -- Acme Corporation users
    ('a1111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 
     'admin@acme.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYbYvJ4wy6C', 
     'John', 'Admin', 'ADMIN', 'IT'),
    ('a2222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 
     'manager.hr@acme.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYbYvJ4wy6C', 
     'Sarah', 'Manager', 'MANAGER', 'HR'),
    ('a3333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', 
     'employee@acme.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYbYvJ4wy6C', 
     'Mike', 'Employee', 'EMPLOYEE', 'SALES'),
    
    -- TechCorp users
    ('b1111111-1111-1111-1111-111111111111', '22222222-2222-2222-2222-222222222222', 
     'admin@techcorp.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYbYvJ4wy6C', 
     'Jane', 'Smith', 'ADMIN', 'IT'),
    ('b2222222-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 
     'cfo@techcorp.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYbYvJ4wy6C', 
     'Robert', 'CFO', 'MANAGER', 'FINANCE');

-- Insert workflow configurations
INSERT INTO workflow_configs (tenant_id, department, workflow_type, name, description, approval_chain, required_approvals) VALUES
    -- Acme HR workflows
    ('11111111-1111-1111-1111-111111111111', 'HR', 'LEAVE_REQUEST', 'Leave Request Workflow', 
     'Standard leave approval process', 
     '[{"role": "MANAGER", "department": "HR"}]', 1),
    
    -- Acme Finance workflows
    ('11111111-1111-1111-1111-111111111111', 'FINANCE', 'EXPENSE_CLAIM', 'Expense Claim Workflow', 
     'Two-tier expense approval', 
     '[{"role": "MANAGER", "department": "FINANCE"}, {"role": "ADMIN"}]', 2),
    
    -- TechCorp HR workflows
    ('22222222-2222-2222-2222-222222222222', 'HR', 'LEAVE_REQUEST', 'Leave Request Workflow', 
     'Leave approval with single step', 
     '[{"role": "MANAGER", "department": "HR"}]', 1);

-- Insert sample workflows
INSERT INTO workflows (id, tenant_id, config_id, created_by, title, description, status, data) VALUES
    ('w1111111-1111-1111-1111-111111111111', 
     '11111111-1111-1111-1111-111111111111',
     (SELECT id FROM workflow_configs WHERE tenant_id = '11111111-1111-1111-1111-111111111111' AND workflow_type = 'LEAVE_REQUEST'),
     'a3333333-3333-3333-3333-333333333333',
     'Vacation Leave Request',
     'Requesting 5 days off for vacation',
     'PENDING',
     '{"leave_type": "vacation", "start_date": "2025-03-15", "end_date": "2025-03-20", "days": 5}');

-- Create audit log for workflow creation
INSERT INTO audit_logs (tenant_id, user_id, entity_type, entity_id, action, new_values) VALUES
    ('11111111-1111-1111-1111-111111111111',
     'a3333333-3333-3333-3333-333333333333',
     'WORKFLOW',
     'w1111111-1111-1111-1111-111111111111',
     'CREATED',
     '{"title": "Vacation Leave Request", "status": "PENDING"}');

-- ============================================================================
-- UTILITY FUNCTIONS
-- ============================================================================

-- Function to set tenant context (called by middleware)
CREATE OR REPLACE FUNCTION set_tenant_context(tenant_uuid UUID)
RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_tenant_id', tenant_uuid::TEXT, TRUE);
END;
$$ LANGUAGE plpgsql;

-- Function to get current tenant
CREATE OR REPLACE FUNCTION get_current_tenant()
RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_tenant_id', TRUE)::UUID;
EXCEPTION WHEN OTHERS THEN
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Grant necessary permissions
GRANT USAGE ON SCHEMA public TO PUBLIC;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO PUBLIC;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO PUBLIC;
