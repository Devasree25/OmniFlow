import { Component, OnInit } from '@angular/core';
import { WorkflowService, Workflow } from '@core/services/workflow.service';

@Component({
  selector: 'app-workflow-list',
  template: `
    <div class="workflow-list-container">
      <h2>Workflows</h2>
      <div class="loading" *ngIf="loading">
        <div class="spinner"></div>
      </div>
      <div *ngIf="!loading">
        <p>{{ workflows.length }} workflows found</p>
        <div *ngFor="let workflow of workflows" class="card" style="margin-bottom: 1rem;">
          <h3>{{ workflow.title }}</h3>
          <p>Status: {{ workflow.status }}</p>
        </div>
      </div>
    </div>
  `,
  styles: [`.workflow-list-container { padding: 2rem; }`]
})
export class WorkflowListComponent implements OnInit {
  workflows: Workflow[] = [];
  loading = true;

  constructor(private workflowService: WorkflowService) {}

  ngOnInit(): void {
    this.workflowService.getWorkflows().subscribe({
      next: (data) => {
        this.workflows = data;
        this.loading = false;
      },
      error: () => {
        this.loading = false;
      }
    });
  }
}
