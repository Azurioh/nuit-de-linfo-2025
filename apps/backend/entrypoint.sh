#!/bin/bash
set -e

# Generate self-signed certificate if not exists AND we are in production
if [ "$APP_ENV" = "production" ] && ([ ! -f cert.pem ] || [ ! -f key.pem ]); then
    echo "Generating self-signed certificate for HTTP/2 (Production)..."
    openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
else
    echo "Skipping certificate generation (APP_ENV=$APP_ENV). Starting in HTTP/1.1 mode."
fi

# Execute the main command
exec "$@"
