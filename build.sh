#! /bin/bash

echo " ----  INSTALLING RUST  ---- "
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo " ----     GIT CLONE     ---- "
git clone --depth 1 https://github.com/th3Maid/MaidRunner.git --single-branch --branch master

echo " ---- BUILD AND INSTALL ---- "
sudo su
mkdir /var/maid
cp maid_lists/ /var/maid -R
chown $(whoami):$(whoami) /var/maid -R
apt-get update
apt-get install -y nmap curl xxd libc6 exiftool traceroute wget iproute2 whois dnsutils dirb dnsenum