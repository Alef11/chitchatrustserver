#!/bin/bash

# Exit on any error
set -e

ENV_FILE=".env"

# Check if .env file exists
if [ -f "$ENV_FILE" ]; then
    echo ".env file already exists."
    read -p "Do you want to overwrite it? [y/N]: " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo "Aborting without changes."
        exit 0
    fi
fi

# Prompt the user for required values
read -p "Enter MARIADB_ROOT_PASSWORD: " MARIADB_ROOT_PASSWORD
read -p "Enter MARIADB_USER: " MARIADB_USER
read -p "Enter MARIADB_PASSWORD: " MARIADB_PASSWORD
read -p "Enter PUBLIC_DOMAIN (e.g., example.com): " PUBLIC_DOMAIN

# Prompt for optional values with defaults
read -p "Enter MARIADB_HOST [default: localhost]: " MARIADB_HOST
MARIADB_HOST=${MARIADB_HOST:-localhost}

read -p "Enter MARIADB_PORT [default: 3306]: " MARIADB_PORT
MARIADB_PORT=${MARIADB_PORT:-3306}

read -p "Enter MARIADB_DATABASE [default: chitchat_db]: " MARIADB_DATABASE
MARIADB_DATABASE=${MARIADB_DATABASE:-chitchat_db}

# Write to .env file
cat <<EOF > "$ENV_FILE"
MARIADB_ROOT_PASSWORD=$MARIADB_ROOT_PASSWORD
MARIADB_USER=$MARIADB_USER
MARIADB_PASSWORD=$MARIADB_PASSWORD
MARIADB_HOST=$MARIADB_HOST
MARIADB_PORT=$MARIADB_PORT
MARIADB_DATABASE=$MARIADB_DATABASE
PUBLIC_DOMAIN=$PUBLIC_DOMAIN
EOF

echo ".env file created successfully. ✅"
