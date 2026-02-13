Alright. Here is your README rewritten in a clean, brief, professional format — no unnecessary fluff, straight to the point.

OmniFlow Platform

Multi-Tenant SaaS Workflow System

A production-ready SaaS platform built with Angular (frontend) and Rust + Axum (backend) using PostgreSQL with Row-Level Security (RLS) for secure multi-tenant workflow management.

Tech Stack

Frontend: Angular 17+, RxJS

Backend: Rust (Axum)

Database: PostgreSQL 15+

Auth: JWT-based authentication

Multi-Tenancy: tenant_id + PostgreSQL RLS

Prerequisites

Install:

Node.js (v18+)

Angular CLI (v17+)

Rust (latest stable)

PostgreSQL (v15+)

Git

Optional: Docker, VS Code extensions (Angular LS, rust-analyzer, Docker, GitLens)

Quick Start
1️⃣ Setup Database
docker run --name omniflow-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=omniflow \
  -p 5432:5432 \
  -d postgres:15

Or manually create:

CREATE DATABASE omniflow;

Run migrations:

cd backend
psql -U postgres -d omniflow -f migrations/001_initial_schema.sql
2️⃣ Run Backend
cd backend
cargo build
cp .env.example .env
# Update DATABASE_URL and JWT_SECRET
cargo run

Backend runs at:
http://localhost:3000

3️⃣ Run Frontend
cd frontend
npm install
npm start

Frontend runs at:
http://localhost:4200

Demo Login

admin@acme.com
 / password123

admin@techcorp.com
 / password123

Project Structure
backend/     → Rust API
frontend/    → Angular app
migrations/  → SQL schema files

Backend contains:

Models

Routes

Middleware

Services

DB layer

Frontend contains:

Core services

Shared components

Feature modules (Admin, HR, Finance)

Key Features

Multi-tenant isolation (PostgreSQL RLS)

JWT Authentication

Role-Based Access Control (RBAC)

Configurable approval workflows

Audit logging

JSONB workflow configuration

Workflow States

PENDING → APPROVED → COMPLETED

PENDING → REJECTED

Examples:

HR Leave (1 approval)

Finance Expense (2 approvals)

Testing

Backend:

cargo test

Frontend:

npm test
npm run e2e

Docker Deployment
docker-compose up --build

Last updated: 2026-02-13