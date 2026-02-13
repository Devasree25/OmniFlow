import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';
import { ReactiveFormsModule } from '@angular/forms';
import { WorkflowListComponent } from './workflow-list/workflow-list.component';

const routes: Routes = [
  { path: '', component: WorkflowListComponent },
];

@NgModule({
  declarations: [WorkflowListComponent],
  imports: [CommonModule, ReactiveFormsModule, RouterModule.forChild(routes)],
})
export class WorkflowsModule {}
