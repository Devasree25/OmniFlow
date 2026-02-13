# OmniFlow Platform - Multi-Tenant SaaS Workflow System

A production-ready multi-tenant SaaS platform combining Angular's architectural strength with Rust's runtime safety for institutional workflows.

## 🏗️ Architecture Overview

- **Frontend**: Angular 17+ with RxJS for reactive state management
- **Backend**: Rust (Axum framework) for high-performance API
- **Database**: PostgreSQL 15+ with Row-Level Security (RLS)
- **Multi-Tenancy**: Logical isolation via tenant_id + RLS policies

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

### Required Software

1. **Node.js & npm** (v18 or higher)
   - Download from: https://nodejs.org/
   - Verify: `node --version` and `npm --version`

2. **Rust** (latest stable)
   - Install via rustup: https://rustup.rs/
   - Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Verify: `rustc --version` and `cargo --version`

3. **PostgreSQL** (v15 or higher)
   - Download from: https://www.postgresql.org/download/
   - Or use Docker: `docker run --name omniflow-db -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:15`
   - Verify: `psql --version`

4. **Angular CLI** (v17 or higher)
   - Install globally: `npm install -g @angular/cli`
   - Verify: `ng version`

5. **Git** (for version control)
   - Download from: https://git-scm.com/
   - Verify: `git --version`

### Optional but Recommended

- **VS Code Extensions**:
  - Angular Language Service
  - rust-analyzer
  - PostgreSQL (Chris Kolkman)
  - Docker (Microsoft)
  - GitLens

## 🚀 Quick Start Guide

### Step 1: Setup PostgreSQL Database

```bash
# Start PostgreSQL (if using Docker)
docker run --name omniflow-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=omniflow \
  -p 5432:5432 \
  -d postgres:15

# Or connect to your local PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE omniflow;
```

### Step 2: Initialize Database Schema

```bash
cd backend
psql -U postgres -d omniflow -f migrations/001_initial_schema.sql
```

### Step 3: Setup Backend (Rust)

```bash
cd backend

# Install dependencies and build
cargo build

# Create .env file
cp .env.example .env

# Edit .env with your database credentials
# DATABASE_URL=postgresql://postgres:postgres@localhost:5432/omniflow
# JWT_SECRET=your-super-secret-jwt-key-change-this-in-production

# Run the backend
cargo run
```

The backend will start on `http://localhost:3000`

### Step 4: Setup Frontend (Angular)

```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm start
```

The frontend will start on `http://localhost:4200`

### Step 5: Access the Application

1. Open browser: `http://localhost:4200`
2. Login with demo credentials:
   - **Tenant 1 Admin**: `admin@acme.com` / `password123`
   - **Tenant 2 Admin**: `admin@techcorp.com` / `password123`

## 📁 Project Structure

```
omniflow-platform/
├── backend/                    # Rust API (Axum)
│   ├── src/
│   │   ├── main.rs            # Entry point
│   │   ├── models/            # Data models
│   │   ├── routes/            # API endpoints
│   │   ├── middleware/        # Tenant isolation
│   │   ├── services/          # Business logic
│   │   └── db/                # Database layer
│   ├── migrations/            # SQL migrations
│   ├── Cargo.toml            # Rust dependencies
│   └── .env.example          # Environment template
│
├── frontend/                  # Angular app
│   ├── src/
│   │   ├── app/
│   │   │   ├── core/         # Singletons (auth, guards)
│   │   │   ├── shared/       # Reusable components
│   │   │   ├── features/     # Feature modules
│   │   │   │   ├── admin/    # Admin workflows
│   │   │   │   ├── hr/       # HR department
│   │   │   │   └── finance/  # Finance department
│   │   │   └── app.component.ts
│   │   └── environments/     # Environment configs
│   ├── angular.json
│   ├── package.json
│   └── tsconfig.json
│
└── README.md                 # This file
```

## 🔐 Security Features

1. **Row-Level Security (RLS)**: PostgreSQL enforces tenant isolation at database level
2. **JWT Authentication**: Secure token-based auth with tenant_id embedded
3. **RBAC Guards**: Angular route guards prevent unauthorized access
4. **Audit Logging**: Every workflow transition is recorded immutably

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
npm test                # Unit tests
npm run e2e            # End-to-end tests
```

## 🔄 Workflow System

The platform supports configurable approval workflows:

1. **HR Leave Request**: Single approval (Manager)
2. **Finance Expense**: Two approvals (Manager → CFO)
3. **Custom Workflows**: Define via Admin panel (JSONB config)

### Workflow States
- `PENDING` → `APPROVED` → `COMPLETED`
- `PENDING` → `REJECTED`

## 📊 Database Schema

Key tables:
- `tenants`: Institution/organization info
- `users`: User accounts with role/department
- `workflows`: Workflow instances
- `workflow_configs`: Department-specific rules (JSONB)
- `audit_logs`: Immutable event log

## 🐳 Docker Deployment

```bash
# Build all services
docker-compose up --build

# Or individually
docker build -t omniflow-backend ./backend
docker build -t omniflow-frontend ./frontend
```

## 🛠️ Development Workflow

1. **Backend changes**: 
   - Edit Rust code in `backend/src/`
   - Run `cargo run` (auto-reloads with `cargo-watch`)

2. **Frontend changes**:
   - Edit Angular code in `frontend/src/`
   - Auto-reloads via `npm start`

3. **Database changes**:
   - Create new migration in `backend/migrations/`
   - Apply: `psql -U postgres -d omniflow -f migrations/XXX_name.sql`

## 🐛 Troubleshooting

### Backend won't start
- Check PostgreSQL is running: `pg_isready`
- Verify .env file exists with correct DATABASE_URL
- Check port 3000 is available: `lsof -i :3000`

### Frontend build errors
- Delete `node_modules` and reinstall: `rm -rf node_modules && npm install`
- Clear Angular cache: `ng cache clean`

### Database connection fails
- Verify PostgreSQL credentials in .env
- Check if database exists: `psql -U postgres -l`
- Ensure migrations ran successfully

## 📚 Learning Resources

- **Angular**: https://angular.io/docs
- **Rust/Axum**: https://docs.rs/axum/latest/axum/
- **PostgreSQL RLS**: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
- **RxJS**: https://rxjs.dev/guide/overview

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Support

- Open an issue on GitHub
- Email: support@omniflow.example
- Documentation: https://docs.omniflow.example

---

**Built with ❤️ using Angular + Rust + PostgreSQL**
