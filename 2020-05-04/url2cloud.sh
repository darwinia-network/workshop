#!/usr/bin/bash
sleep 2
sed -i 's/^.*eth-resource//g' /tmp/hosts
echo '107.167.191.203 eth-resource' >> /tmp/hosts
