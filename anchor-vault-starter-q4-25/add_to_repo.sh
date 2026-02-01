#!/bin/bash

# Script to add anchor-vault-starter-q4-25 to Q1_26_AsyncBuilder_Mikey repository

echo "Copying anchor-vault-starter-q4-25 to Q1_26_AsyncBuilder_Mikey repository..."
cp -r /home/mikey/anchor-vault-starter-q4-25/anchor-vault-starter-q4-25 /home/mikey/solana-starter/

echo "Navigating to repository..."
cd /home/mikey/solana-starter

echo "Checking git status..."
git status

echo "Adding the new project..."
git add anchor-vault-starter-q4-25

echo "Committing changes..."
git commit -m "Add anchor-vault-starter-q4-25 project with withdraw and close implementations"

echo "Pushing to GitHub..."
git push origin master

echo "Done! Project has been added to your GitHub repository."
