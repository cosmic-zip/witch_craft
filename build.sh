#! /bin/bash
echo " ---- BUILD AND INSTALL ---- "

sudo su
mkdir /var/maid
cp maid_lists/ /var/maid -R
chown $(whoami):$(whoami) /var/maid -R
apt-get update
apt-get install -y nmap curl xxd libc6 exiftool traceroute wget iproute2 whois dnsutils dirb dnsenum