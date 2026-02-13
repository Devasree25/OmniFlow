# OmniFlow Platform - Technical Architecture

## System Overview

OmniFlow is a multi-tenant SaaS platform designed for institutional workflows with enterprise-grade security and scalability.

### Core Principles

1. **Multi-Tenancy**: Logical isolation using PostgreSQL Row-Level Security (RLS)
2. **Type Safety**: Rust's compile-time guarantees prevent runtime errors
3. **Reactive State**: RxJS streams for real-time workflow updates
4. **Zero Trust**: Every database query is tenant-filtered at the database level

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        Browser (Client)                      │
│                     Angular 17 + RxJS                        │
└─────────────────────┬───────────────────────────────────────┘
                      │ HTTP/REST + JWT
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                   Rust Backend (Axum)                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tenant Context Middleware                           │   │
│  │  - Extracts tenant_id from JWT                       │   │
│  │  - Sets PostgreSQL session variable                  │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Routes Layer                                        │   │
│  │  - Auth (login, register)                           │   │
│  │  - Workflows (CRUD, approve, reject)                │   │
│  │  - Users (list, get)                                │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Services Layer                                      │   │
│  │  - Workflow Engine (state machine)                  │   │
│  │  - Audit Service (immutable logs)                   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────┬───────────────────────────────────────┘
                      │ SQLx (async)
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                PostgreSQL 15 with RLS                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Row-Level Security Policies                        │   │
│  │  WHERE tenant_id = current_setting('app.tenant_id') │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Tables: tenants, users, workflows,                 │   │
│  │          workflow_configs, audit_logs               │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Backend: Rust + Axum

**Why Rust?**
- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency
- Excellent async/await support

**Key Dependencies:**
- `axum`: Web framework (ergonomic, fast, Tower ecosystem)
- `sqlx`: Compile-time checked SQL queries
- `tokio`: Async runtime
- `jsonwebtoken`: JWT authentication
- `bcrypt`: Password hashing

### Frontend: Angular 17

**Why Angular?**
- Strict TypeScript for enterprise codebases
- Built-in dependency injection
- RxJS for reactive state management
- Powerful CLI and module system

**Architecture Pattern:**
```
app/
├── core/           # Singletons (AuthService, Guards)
├── shared/         # Reusable components/pipes
└── features/       # Lazy-loaded feature modules
    ├── auth/       # Login, registration
    ├── dashboard/  # Main dashboard
    ├── workflows/  # Workflow management
    └── admin/      # Admin panel
```

### Database: PostgreSQL 15

**Why PostgreSQL?**
- Row-Level Security (RLS) for tenant isolation
- JSONB for flexible workflow data
- Excellent performance with proper indexing
- ACID compliance

## Multi-Tenancy Implementation

### 1. Tenant Context Middleware (Rust)

```rust
pub async fn tenant_context_middleware(...) {
    let tenant_id = extract_from_jwt(token);
    
    // This is the "magic" - set session variable
    sqlx::query(&format!(
        "SET LOCAL app.current_tenant_id = '{}'", 
        tenant_id
    )).execute(&db).await;
}
```

### 2. Row-Level Security Policies (PostgreSQL)

```sql
CREATE POLICY tenant_isolation_workflows ON workflows
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);
```

**Critical Insight:**
Even if a developer forgets to add `WHERE tenant_id = ?` in a query, RLS ensures tenant isolation at the database level. This is defense-in-depth.

### 3. JWT Claims

```typescript
interface Claims {
  sub: UUID;        // user_id
  tenant_id: UUID;  // THE KEY
  email: string;
  role: UserRole;
  exp: number;
}
```

## Workflow State Machine

### States

```
PENDING → IN_REVIEW → APPROVED → COMPLETED
  ↓
REJECTED
```

### Approval Chain (JSONB)

```json
{
  "approval_chain": [
    { "role": "MANAGER", "department": "HR" },
    { "role": "ADMIN" }
  ],
  "required_approvals": 2
}
```

### Engine Logic (Pseudocode)

```
1. User creates workflow
2. System fetches workflow_config for that department/type
3. For each step in approval_chain:
   a. Create workflow_approval record (status: PENDING)
   b. Notify designated approver
4. When approver acts:
   a. Update workflow_approval status
   b. If all required approvals met → APPROVED
   c. If any rejection → REJECTED
5. Create audit log for every state change
```

## Security Layers

### Layer 1: Network
- HTTPS/TLS in production
- CORS configured for specific origins

### Layer 2: Application
- JWT authentication
- HTTP-only cookies (optional)
- Role-based access control (RBAC) in Angular guards

### Layer 3: Database
- Row-Level Security (RLS)
- Prepared statements (SQL injection prevention via SQLx)
- Audit logs (immutable, append-only)

### Layer 4: Code
- Rust's borrow checker prevents memory vulnerabilities
- TypeScript's type system catches errors at compile-time

## Data Flow Example: Creating a Workflow

```
1. USER clicks "Create Leave Request" in Angular

2. ANGULAR component calls:
   workflowService.createWorkflow({
     config_id: 'hr-leave-uuid',
     title: 'Vacation',
     data: { start: '2025-03-15', days: 5 }
   })

3. HTTP INTERCEPTOR adds JWT header:
   Authorization: Bearer eyJ...

4. RUST MIDDLEWARE extracts tenant_id from JWT:
   tenant_id = 'acme-uuid'
   
5. MIDDLEWARE sets PostgreSQL session:
   SET LOCAL app.current_tenant_id = 'acme-uuid'

6. ROUTE HANDLER inserts workflow:
   INSERT INTO workflows (...) VALUES (...)
   -- RLS ensures tenant_id is automatically set

7. AUDIT SERVICE logs the action:
   INSERT INTO audit_logs (entity_type: 'WORKFLOW', action: 'CREATED')

8. RESPONSE sent to frontend:
   { id: 'new-workflow-uuid', status: 'PENDING', ... }

9. ANGULAR updates observable stream:
   workflowsSubject.next([...workflows, newWorkflow])

10. UI auto-updates via RxJS subscription
```

## Performance Considerations

### Backend
- Connection pooling (SQLx)
- Async/await for non-blocking I/O
- Compiled binary (vs interpreted languages)

### Frontend
- Lazy-loaded modules
- OnPush change detection strategy (recommended)
- RxJS operators (debounce, distinctUntilChanged)

### Database
- Indexes on tenant_id, created_at, status
- JSONB indexes for workflow data queries
- Materialized views for analytics (future)

## Scalability Path

### Current (MVP)
- Single PostgreSQL instance
- Single Rust binary (can handle 10k+ req/sec)
- Static Angular files via Nginx

### Future
- Read replicas for PostgreSQL
- Horizontal scaling of Rust instances (stateless)
- Redis for session caching
- Message queue (RabbitMQ) for async workflows
- CDN for Angular static assets

## Monitoring & Observability

### Recommended Tools
- **Logs**: `tracing` crate (Rust), ELK stack
- **Metrics**: Prometheus + Grafana
- **Tracing**: OpenTelemetry
- **Errors**: Sentry

### Key Metrics
- Request latency (p50, p95, p99)
- Database query time
- Workflow completion rate
- Tenant-specific usage patterns

## Development Best Practices

### Rust
- Use `cargo clippy` for lints
- Run `cargo test` before commits
- Keep route handlers thin, logic in services

### Angular
- Follow Angular style guide
- Use `ng lint` for consistency
- Prefer reactive forms over template-driven
- Unsubscribe from observables (or use async pipe)

### Database
- Never bypass RLS in application code
- Always use prepared statements
- Test migrations on staging first
- Keep audit logs forever (compliance)

## Testing Strategy

### Unit Tests
- Rust: `cargo test`
- Angular: `ng test` (Jasmine + Karma)

### Integration Tests
- API: `reqwest` library in Rust
- E2E: Playwright or Cypress for Angular

### Security Tests
- SQL injection attempts (should fail via SQLx)
- Cross-tenant access attempts (should fail via RLS)
- JWT tampering (should fail signature verification)

## Deployment

### Development
```bash
# Backend
cd backend && cargo run

# Frontend
cd frontend && npm start
```

### Production (Docker)
```bash
docker-compose up -d
```

### Production (Manual)
```bash
# Backend
cargo build --release
./target/release/omniflow-backend

# Frontend
ng build --configuration production
# Serve dist/ via Nginx
```

## Future Enhancements

1. **Real-time Updates**: WebSocket support for live workflow status
2. **Advanced Workflows**: Conditional branching, parallel approvals
3. **Notifications**: Email/Slack integration
4. **Analytics Dashboard**: Workflow metrics per tenant
5. **API Keys**: Machine-to-machine authentication
6. **Webhook Support**: Trigger external systems on workflow events

## References

- **Axum Documentation**: https://docs.rs/axum/
- **SQLx Guide**: https://github.com/launchbadge/sqlx
- **Angular Docs**: https://angular.io/docs
- **PostgreSQL RLS**: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
- **JWT Best Practices**: https://tools.ietf.org/html/rfc8725

---

**Built with ❤️ for enterprise-grade workflows**
