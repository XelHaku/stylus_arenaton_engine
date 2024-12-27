#!/bin/bash
# sudo chown -R "$USER":"$USER" target
# Enable verbose output
set -x

# Display current working directory
echo "Current directory: $(pwd)"

# Clone the nitro-devnode repository if it doesn't already exist
if [ ! -d "nitro-devnode" ]; then
    git clone https://github.com/OffchainLabs/nitro-devnode.git
else
    echo "Directory nitro-devnode already exists. Skipping clone."
fi

cd nitro-devnode || exit

# Stop and remove existing nitro-dev container if it exists
echo "Stopping and removing any existing nitro-dev container..."
docker stop nitro-dev || true
docker rm nitro-dev || true

# Launch the devnode in the background
nohup ./run-dev-node.sh >/dev/null 2>&1 &

# Return to the parent directory
cd .. || exit

# Function to deploy a project if the directory exists
deploy_project() {
    local dir=$1
    local key=$2
    if [ -d "$dir" ]; then
        echo "Deploying $dir..."
        cd "$dir" || exit
        cargo stylus deploy --private-key "$key"
        cd .. || exit
    else
        echo "Directory $dir not found. Skipping deployment."
    fi
}

# Nitro Node private key
PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Deploy projects
deploy_project "stylus_erc20aton" "$PRIVATE_KEY"
deploy_project "stylus_vault" "$PRIVATE_KEY"
deploy_project "stylus_core_events" "$PRIVATE_KEY"
deploy_project "stylus_stake_engine" "$PRIVATE_KEY"
