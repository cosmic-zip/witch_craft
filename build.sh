#! /bin/bash

echo "#####################################"
echo "#  Don't run this script with sudo  #"
echo "#####################################"
echo
echo "INSTALLING RUST"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo
echo "INSTALL DEPENDENCIES"
sudo apt install nala -y 
sudo mkdir /var/maid
sudo cp maid_lists/ /var/maid -R
sudo chown $(whoami):$(whoami) /var/maid -R
sudo nala update
sudo nala install -y nmap curl xxd libc6 exiftool traceroute wget iproute2 whois dnsutils dirb dnsenum tree htop tcpdump openssl iftop

echo
echo "BUILD AND INSTALL"
cargo build --release --verbose --manifest-path maid_runner/Cargo.toml
sudo cp ./maid_runner/target/release/maid_runner /bin