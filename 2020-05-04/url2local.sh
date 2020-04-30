#!/usr/bin/bash
sleep 2
sed -i 's/^.*eth-resource//g' /tmp/hosts
echo '127.0.0.1 eth-resource' >> /tmp/hosts
