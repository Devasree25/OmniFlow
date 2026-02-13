import { Component, OnInit } from '@angular/core';
import { AuthService, User } from '@core/services/auth.service';
import { WorkflowService, Workflow } from '@core/services/workflow.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss'],
})
export class DashboardComponent implements OnInit {
  currentUser: User | null = null;
  workflows: Workflow[] = [];
  loading = true;

  constructor(
    private authService: AuthService,
    private workflowService: WorkflowService,
    private router: Router
  ) {}

  ngOnInit(): void {
    this.currentUser = this.authService.getCurrentUser();
    this.loadWorkflows();
  }

  loadWorkflows(): void {
    this.workflowService.getWorkflows().subscribe({
      next: (workflows) => {
        this.workflows = workflows;
        this.loading = false;
      },
      error: (err) => {
        console.error('Error loading workflows:', err);
        this.loading = false;
      },
    });
  }

  logout(): void {
    this.authService.logout();
  }

  getStatusBadgeClass(status: string): string {
    const classMap: any = {
      PENDING: 'badge-pending',
      IN_REVIEW: 'badge-in-review',
      APPROVED: 'badge-approved',
      REJECTED: 'badge-rejected',
    };
    return classMap[status] || 'badge-pending';
  }
}
