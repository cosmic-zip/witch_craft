#!/bin/bash
set -e

echo && echo "Install dependencies"
sudo apt update
sudo apt install -y nmap whois dirb dnsenum libc-bin iproute2 xxd iptables coreutils wget curl \
dnsutils traceroute openssl openssh-server xattr libimage-exiftool-perl tor foremost pkg-config \
libssl-dev steghide doas nala libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev 7zip \
libayatana-appindicator3-dev librsvg2-dev

# Install data
echo && echo "Install witch_spells data"
sudo cp -r archive/ /var/
sudo chown -R $(whoami):$(whoami) /var/archive
7z x /var/archive/witch_spells/osint/archive.7z.001
echo 'export WITCH_SPELLS_ROOT_DIR=/var/archive/witch_spells/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/archive/witch_spells/

# Wordlist
echo && echo "Wordlists are big (16GB) so, if you want or need then, just run:"
echo "7z x /var/witch_craft/witch_spells/wordlists/ladybug.pwned"
echo "7z x /var/witch_craft/witch_spells/wordlists/moth.pwned.7z.001"

# Build binary
echo && echo "Cargo build"
cargo build --release --manifest-path witch_craft/Cargo.toml
chmod +x ./witch_craft/target/release/witch_craft
sudo cp -r ./witch_craft/target/release/witch_craft /bin
