# Get the REGISTRY_URL secret from Doppler
$REGISTRY_URL = & doppler secrets get REGISTRY_URL --plain

if (-not $REGISTRY_URL) {
    Write-Host "REGISTRY_URL is not set"
    exit 1
}

# Build the Docker image
docker build -t connection-holder .

# Tag the image with the registry URL
docker tag connection-holder "$REGISTRY_URL/connection-holder:latest"

# Push the image to the registry
docker push "$REGISTRY_URL/connection-holder:latest"
