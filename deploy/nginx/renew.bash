#!/bin/bash

COMPOSE="/usr/local/bin/docker-compose --no-ansi"
DOCKER="/usr/bin/docker"

$COMPOSE run certbot renew --dry-run && $COMPOSE kill -s SIGHUP nginx
$DOCKER system prune -af

sudo openssl dhparam -out /home/julie/.ssl/dhparam/dhparam-2048.pem 2048
