#!/bin/bash
set -e

# Install deps
echo && echo "Install dependencies"
sudo apt update
sudo apt install -y nmap whois dirb dnsenum libc-bin iproute2 xxd iptables coreutils wget curl \
dnsutils traceroute openssl openssh-server xattr libimage-exiftool-perl tor foremost pkg-config \
libssl-dev steghide libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev 7zip \
libayatana-appindicator3-dev librsvg2-dev chromium

# Install data
echo && echo "Install witch_spells data"
sudo cp -r witch_spells/ /var/
sudo chown -R $(whoami):$(whoami) /var/witch_spells
echo "Run: 7z x /var/witch_spells/archive/osint/Archive.7z.001 -o/var/witch_spells/archive/osint/"
echo 'export WITCH_SPELLS_ROOT_DIR=/var/witch_spells/archive/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/witch_spells/archive/

# Wordlist
echo && echo "Wordlists are big (16GB) so, if you want or need then, just run:"
echo "7z x /var/witch_craft/witch_spells/wordlists/ladybug.pwned"
echo "7z x /var/witch_craft/witch_spells/wordlists/moth.pwned.7z.001"

# Build binary
echo && echo "Cargo build"
cargo build --release --manifest-path witch_craft/Cargo.toml
chmod +x ./witch_craft/target/release/witch_craft
sudo cp -r ./witch_craft/target/release/witch_craft /bin
