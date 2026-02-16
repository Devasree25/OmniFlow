# Create a new branch, commit changes, and push so you can open a Pull Request on GitHub.
# Run from project root: .\create-pull-request.ps1

$branchName = "feature/dashboard-and-readme-updates"

Write-Host "Creating branch: $branchName" -ForegroundColor Cyan
git checkout -b $branchName 2>$null
if ($LASTEXITCODE -ne 0) {
    # Branch might already exist, switch to it
    git checkout $branchName
}

Write-Host "Staging README and dashboard UI changes..." -ForegroundColor Cyan
git add README.md frontend/src/app/features/dashboard/dashboard.component.html

Write-Host "Committing..." -ForegroundColor Cyan
git commit -m "docs: update README and dashboard welcome copy"

Write-Host "Pushing branch to origin (this will allow a Pull Request)..." -ForegroundColor Cyan
git push -u origin $branchName

Write-Host ""
Write-Host "Done. Go to your GitHub repo and you should see: 'Compare & pull request' for $branchName" -ForegroundColor Green
