import { Component } from '@angular/core';

@Component({
  selector: 'app-admin-dashboard',
  template: `
    <div class="admin-container">
      <h2>Admin Panel</h2>
      <div class="card">
        <h3>Workflow Configuration</h3>
        <p>Configure approval chains and workflow rules for your organization.</p>
        <button class="btn btn-primary">Add Workflow Config</button>
      </div>
      <div class="card" style="margin-top: 1rem;">
        <h3>User Management</h3>
        <p>Manage user roles and permissions.</p>
        <button class="btn btn-primary">Manage Users</button>
      </div>
    </div>
  `,
  styles: [`.admin-container { padding: 2rem; } .card { margin-bottom: 1rem; }`]
})
export class AdminDashboardComponent {}
