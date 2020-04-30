#!/usr/bin/bash
sleep 1
kill -s 9 `pidof nginx` || echo "no nginx exsist"
sleep 1
/usr/bin/nginx -c /etc/nginx/nginx.conf
