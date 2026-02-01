#!/bin/bash

# Script to fix the git submodule issue and add as regular files

echo "Navigating to repository..."
cd /home/mikey/solana-starter

echo "Removing the submodule reference..."
git rm --cached anchor-vault-starter-q4-25

echo "Removing the .git folder from anchor-vault-starter-q4-25..."
rm -rf anchor-vault-starter-q4-25/.git

echo "Adding the project as regular files..."
git add anchor-vault-starter-q4-25

echo "Committing changes..."
git commit -m "Fix: Add anchor-vault-starter-q4-25 as regular files (not submodule)"

echo "Pushing to GitHub..."
git push origin master

echo "Done! Project has been properly added to your GitHub repository."
