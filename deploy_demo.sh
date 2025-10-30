#!/bin/bash

# DataGrid5 Demo Deployment Script
# This script builds and deploys the demo to GitHub Pages (gh-pages branch)

set -e  # Exit on error

echo "🚀 DataGrid5 Demo Deployment"
echo "============================"
echo ""

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ Error: Must be on 'main' branch to deploy"
    echo "   Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: You have uncommitted changes"
    read -p "   Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Deployment cancelled"
        exit 1
    fi
fi

# Build the project
echo "📦 Building WASM project..."
./build.sh --release

if [ ! -f "pkg/datagrid5_bg.wasm" ]; then
    echo "❌ Error: Build failed - WASM file not found"
    exit 1
fi

echo "✅ Build completed successfully"
echo ""

# Stash any uncommitted changes
HAS_STASH=false
if [ -n "$(git status --porcelain)" ]; then
    echo "💾 Stashing uncommitted changes..."
    git stash push -m "deploy_demo.sh: temporary stash"
    HAS_STASH=true
fi

# Switch to gh-pages branch
echo "🔄 Switching to gh-pages branch..."
git checkout gh-pages

# Copy updated files from main branch
echo "📋 Copying files from main branch..."
git checkout main -- examples www

# Add built files (pkg is not tracked in main branch)
echo "📦 Adding built WASM files..."
git add -f pkg/

# Check if there are changes to commit
if [ -z "$(git status --porcelain)" ]; then
    echo "ℹ️  No changes to deploy"
    git checkout main

    # Pop stash if we created one
    if [ "$HAS_STASH" = true ]; then
        echo "♻️  Restoring stashed changes..."
        git stash pop
    fi

    exit 0
fi

# Show what will be deployed
echo ""
echo "📊 Changes to be deployed:"
git status --short
echo ""

# Confirm deployment
read -p "🚀 Deploy these changes to GitHub Pages? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Deployment cancelled"
    echo "🔄 Cleaning up..."
    git reset --hard
    git checkout main

    # Pop stash if we created one
    if [ "$HAS_STASH" = true ]; then
        echo "♻️  Restoring stashed changes..."
        git stash pop
    fi

    exit 1
fi

# Commit and push
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
echo "💾 Committing changes..."
git commit -m "Deploy demo: $TIMESTAMP

Update demo site with latest changes.

🤖 Generated with deploy_demo.sh"

echo "⬆️  Pushing to GitHub..."
git push origin gh-pages

echo ""
echo "✅ Deployment successful!"
echo ""
echo "📍 Your demo will be available at:"
echo "   https://oga5.github.io/datagrid5/"
echo ""
echo "⏱️  It may take a few minutes for GitHub Pages to update."
echo ""

# Switch back to main branch
echo "🔄 Switching back to main branch..."
git checkout main

# Pop stash if we created one
if [ "$HAS_STASH" = true ]; then
    echo "♻️  Restoring stashed changes..."
    git stash pop
fi

echo ""
echo "🎉 Done!"
