# Signs Paperback release artifacts with minisign. Run this locally after building and upload all four files to the GitHub release.

param([string]$ArtifactsDir = ".")
$key = "$env:USERPROFILE\.minisign\paperback.key"
if (-not (Test-Path $key)) {
	Write-Error "Secret key not found at $key. Generate one with minisign -G -p `$env:USERPROFILE\.minisign\paperback.pub -s `$env:USERPROFILE\.minisign\paperback.key"
	exit 1
}
if (-not (Get-Command minisign -ErrorAction SilentlyContinue)) {
	Write-Error "minisign not found. Install with scoop install minisign"
	exit 1
}
$artifacts = @("paperback_setup.exe", "paperback.zip")
$signed = @()
foreach ($file in $artifacts) {
	$path = Join-Path $ArtifactsDir $file
	if (-not (Test-Path $path)) {
		Write-Warning "Skipping $file (not found in $ArtifactsDir)"
		continue
	}
	Write-Host "Signing $file..."
	minisign -S -s $key -m $path
	if ($LASTEXITCODE -ne 0) {
		Write-Error "Failed to sign $file"
		exit 1
	}
	$signed += $file
	$signed += "$file.minisig"
}
if ($signed.Count -eq 0) {
	Write-Error "No artifacts found to sign in $ArtifactsDir"
	exit 1
}
Write-Host "Done. Upload these files to the GitHub release:"
foreach ($f in $signed) {
	Write-Host "  $(Join-Path $ArtifactsDir $f)"
}
