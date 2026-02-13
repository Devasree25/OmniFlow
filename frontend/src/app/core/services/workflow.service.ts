import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '@environments/environment';

export interface Workflow {
  id: string;
  tenant_id: string;
  config_id: string;
  created_by: string;
  title: string;
  description: string | null;
  status: 'PENDING' | 'IN_REVIEW' | 'APPROVED' | 'REJECTED' | 'COMPLETED';
  current_step: number;
  data: any;
  created_at: string;
  updated_at: string;
  completed_at: string | null;
}

export interface WorkflowConfig {
  id: string;
  tenant_id: string;
  department: string;
  workflow_type: string;
  name: string;
  description: string | null;
  approval_chain: any;
  required_approvals: number;
  settings: any;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateWorkflowRequest {
  config_id: string;
  title: string;
  description?: string;
  data: any;
}

@Injectable({
  providedIn: 'root',
})
export class WorkflowService {
  constructor(private http: HttpClient) {}

  getWorkflows(): Observable<Workflow[]> {
    return this.http.get<Workflow[]>(`${environment.apiUrl}/workflows`);
  }

  getWorkflow(id: string): Observable<Workflow> {
    return this.http.get<Workflow>(`${environment.apiUrl}/workflows/${id}`);
  }

  createWorkflow(request: CreateWorkflowRequest): Observable<Workflow> {
    return this.http.post<Workflow>(`${environment.apiUrl}/workflows`, request);
  }

  approveWorkflow(
    id: string,
    comments?: string
  ): Observable<Workflow> {
    return this.http.post<Workflow>(
      `${environment.apiUrl}/workflows/${id}/approve`,
      { comments }
    );
  }

  rejectWorkflow(
    id: string,
    comments?: string
  ): Observable<Workflow> {
    return this.http.post<Workflow>(
      `${environment.apiUrl}/workflows/${id}/reject`,
      { comments }
    );
  }

  getWorkflowConfigs(): Observable<WorkflowConfig[]> {
    return this.http.get<WorkflowConfig[]>(
      `${environment.apiUrl}/workflow-configs`
    );
  }
}
