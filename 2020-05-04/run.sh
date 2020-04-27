# Enable port redirect
/usr/bin/nginx -c /etc/nginx/nginx.conf &

# Start shadow service
/dargo shadow 3000

# Start darwinia node
/darwinia --dev -leoc=trace  --base-path /tmp/darwinia-develop/alice



