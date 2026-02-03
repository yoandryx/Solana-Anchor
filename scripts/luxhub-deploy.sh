#!/bin/bash
# ============================================
# LuxHub Deployment Helper Script
# ============================================
#
# Usage:
#   ./scripts/luxhub-deploy.sh build          # Build only
#   ./scripts/luxhub-deploy.sh deploy         # Deploy to devnet
#   ./scripts/luxhub-deploy.sh init           # Initialize config
#   ./scripts/luxhub-deploy.sh full           # Build + Deploy + Init
#   ./scripts/luxhub-deploy.sh mainnet-deploy # Deploy to mainnet (careful!)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ANCHOR_ROOT="$(dirname "$SCRIPT_DIR")"
FRONTEND_IDL="$ANCHOR_ROOT/../src/idl/luxhub_marketplace.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════${NC}\n"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Build the program
build() {
    print_header "Building LuxHub Marketplace"
    cd "$ANCHOR_ROOT"
    anchor build

    # Copy IDL to frontend
    if [ -f "target/idl/luxhub_marketplace.json" ]; then
        cp target/idl/luxhub_marketplace.json "$FRONTEND_IDL"
        print_success "IDL copied to frontend"
    fi

    print_success "Build complete"
}

# Deploy to devnet
deploy_devnet() {
    print_header "Deploying to Devnet"
    cd "$ANCHOR_ROOT"

    # Build first
    build

    # Deploy
    anchor deploy --provider.cluster devnet

    # Get program ID
    PROGRAM_ID=$(solana-keygen pubkey target/deploy/luxhub_marketplace-keypair.json)
    print_success "Deployed to devnet: $PROGRAM_ID"

    # Update .env.local if it exists
    ENV_FILE="$ANCHOR_ROOT/../.env.local"
    if [ -f "$ENV_FILE" ]; then
        if grep -q "PROGRAM_ID=" "$ENV_FILE"; then
            sed -i "s/PROGRAM_ID=.*/PROGRAM_ID=$PROGRAM_ID/" "$ENV_FILE"
        else
            echo "PROGRAM_ID=$PROGRAM_ID" >> "$ENV_FILE"
        fi
        print_success "Updated .env.local with PROGRAM_ID"
    fi

    echo ""
    echo "Program ID: $PROGRAM_ID"
    echo "Explorer:   https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
}

# Initialize config using TypeScript
init_config() {
    print_header "Initializing Escrow Config"
    cd "$ANCHOR_ROOT"

    # Check if ts-node is available
    if ! command -v npx &> /dev/null; then
        print_error "npx not found. Install Node.js first."
        exit 1
    fi

    npx ts-node scripts/deploy-and-init.ts --cluster devnet --action init-config
}

# Full deployment (build + deploy + init)
full_deploy() {
    print_header "Full Deployment (Devnet)"
    deploy_devnet
    init_config
    print_success "Full deployment complete!"
}

# Mainnet deployment (with confirmation)
deploy_mainnet() {
    print_header "⚠️  MAINNET DEPLOYMENT ⚠️"

    print_warning "You are about to deploy to MAINNET!"
    print_warning "This will cost real SOL and cannot be undone."
    echo ""
    read -p "Type 'MAINNET' to confirm: " confirmation

    if [ "$confirmation" != "MAINNET" ]; then
        print_error "Deployment cancelled"
        exit 1
    fi

    echo ""
    read -p "Have you completed the MAINNET_LAUNCH_CHECKLIST.md? (yes/no): " checklist

    if [ "$checklist" != "yes" ]; then
        print_error "Please complete the checklist first"
        exit 1
    fi

    print_header "Deploying to Mainnet"
    cd "$ANCHOR_ROOT"

    # Build
    build

    # Deploy
    anchor deploy --provider.cluster mainnet

    PROGRAM_ID=$(solana-keygen pubkey target/deploy/luxhub_marketplace-keypair.json)
    print_success "Deployed to mainnet: $PROGRAM_ID"

    echo ""
    echo "Program ID: $PROGRAM_ID"
    echo "Explorer:   https://explorer.solana.com/address/$PROGRAM_ID"

    print_warning "Remember to initialize the config with the mainnet Squads multisig!"
}

# Show current status
status() {
    print_header "LuxHub Deployment Status"

    cd "$ANCHOR_ROOT"

    echo "Anchor.toml cluster:"
    grep "cluster" Anchor.toml | head -1
    echo ""

    echo "Program keypair:"
    if [ -f "target/deploy/luxhub_marketplace-keypair.json" ]; then
        PROGRAM_ID=$(solana-keygen pubkey target/deploy/luxhub_marketplace-keypair.json)
        echo "  $PROGRAM_ID"
    else
        echo "  Not found (run build first)"
    fi
    echo ""

    echo ".env.local PROGRAM_ID:"
    if [ -f "$ANCHOR_ROOT/../.env.local" ]; then
        grep "PROGRAM_ID" "$ANCHOR_ROOT/../.env.local" || echo "  Not set"
    else
        echo "  .env.local not found"
    fi
    echo ""

    echo "Frontend IDL exists:"
    if [ -f "$FRONTEND_IDL" ]; then
        echo "  Yes"
    else
        echo "  No"
    fi
}

# Show help
show_help() {
    echo "LuxHub Deployment Script"
    echo ""
    echo "Usage: ./scripts/luxhub-deploy.sh <command>"
    echo ""
    echo "Commands:"
    echo "  build           Build the Anchor program"
    echo "  deploy          Deploy to devnet"
    echo "  init            Initialize escrow config on devnet"
    echo "  full            Full deployment (build + deploy + init)"
    echo "  mainnet-deploy  Deploy to mainnet (requires confirmation)"
    echo "  status          Show current deployment status"
    echo "  help            Show this help message"
    echo ""
}

# Main
case "$1" in
    build)
        build
        ;;
    deploy)
        deploy_devnet
        ;;
    init)
        init_config
        ;;
    full)
        full_deploy
        ;;
    mainnet-deploy)
        deploy_mainnet
        ;;
    status)
        status
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        if [ -n "$1" ]; then
            print_error "Unknown command: $1"
            echo ""
        fi
        show_help
        exit 1
        ;;
esac
