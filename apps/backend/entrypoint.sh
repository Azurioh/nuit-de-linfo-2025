#!/bin/bash
set -e

# Generate self-signed certificate if not exists
if [ ! -f cert.pem ] || [ ! -f key.pem ]; then
    echo "Generating self-signed certificate for HTTP/2..."
    openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
fi

# Execute the main command
exec "$@"
