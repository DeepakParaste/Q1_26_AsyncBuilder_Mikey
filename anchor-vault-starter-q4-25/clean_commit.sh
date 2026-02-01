#!/bin/bash

# Script to remove and re-add anchor-vault-starter-q4-25 with a clean commit message

echo "Navigating to repository..."
cd /home/mikey/solana-starter

echo "Removing anchor-vault-starter-q4-25 from git..."
git rm -r anchor-vault-starter-q4-25

echo "Committing the removal..."
git commit -m "Remove anchor-vault-starter-q4-25 for clean re-add"

echo "Removing the directory completely..."
rm -rf anchor-vault-starter-q4-25

echo "Copying fresh anchor-vault-starter-q4-25 without .git folder..."
cp -r /home/mikey/anchor-vault-starter-q4-25/anchor-vault-starter-q4-25 ./
rm -rf anchor-vault-starter-q4-25/.git

echo "Adding the project with clean commit..."
git add anchor-vault-starter-q4-25

echo "Committing with new message..."
git commit -m "Add anchor-vault-starter-q4-25 project"

echo "Pushing to GitHub..."
git push origin master

echo "Done! Project has been re-added with a clean commit message."
