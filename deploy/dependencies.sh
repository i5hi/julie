#!/bin/bash
# run as sudo
ADMIN_USER=debian
COMPOSE_VER=1.28.2

curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
apt-key fingerprint 0EBFCD88
add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/debian \
   $(lsb_release -cs) \
   stable "

apt-get update
apt-get install -y \
    docker-ce docker-ce-cli containerd.io

curl -L "https://github.com/docker/compose/releases/download/$COMPOSE_VER/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
usermod -aG docker $ADMIN_USER
newgrp docker
# chown $ADMIN_USER /usr/local/bin/docker-compose
# chown $ADMIN_USER /usr/bin/docker

apt-get update
