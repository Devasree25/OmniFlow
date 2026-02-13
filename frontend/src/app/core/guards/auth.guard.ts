import { Injectable } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthService } from '../services/auth.service';

@Injectable({
  providedIn: 'root',
})
export class AuthGuard {
  constructor(private authService: AuthService, private router: Router) {}

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): boolean {
    if (this.authService.isAuthenticated()) {
      // Check role-based access if specified
      const requiredRoles = route.data['roles'] as string[];
      if (requiredRoles && !this.authService.hasRole(requiredRoles)) {
        this.router.navigate(['/dashboard']);
        return false;
      }
      return true;
    }

    // Not logged in, redirect to login
    this.router.navigate(['/auth/login'], {
      queryParams: { returnUrl: state.url },
    });
    return false;
  }
}
