import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';

@NgModule({
  declarations: [],
  imports: [CommonModule, RouterModule, ReactiveFormsModule, FormsModule],
  exports: [CommonModule, RouterModule, ReactiveFormsModule, FormsModule],
})
export class SharedModule {}
