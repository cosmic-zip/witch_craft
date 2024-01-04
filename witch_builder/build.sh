#!/bin/bash
set -e

echo "MR Build System"

echo && echo "Install dependencies"
sudo apt update -y
sudo apt install -y aptitude p7zip-full nmap curl xxd libc6 exiftool \
traceroute wget iproute2 whois dnsutils dirb dnsenum tree htop iftop \
clang sudo libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev \
libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev 

echo && echo "Move config folders to /var"
sudo mkdir -p /var/witch_craft
sudo cp -r witch_spells/ /var/witch_craft
sudo chown -R $(whoami):$(whoami) /var/witch_craft

echo && echo "Uncompress files"
7z x /var/witch_craft/witch_spells/malware/malware.csv.7z.001 -o/var/witch_craft/witch_spells/general/
mv /var/witch_craft/witch_spells/general/full.csv /var/witch_craft/witch_spells/general/malware_hash.config

echo && echo "SNAP Setup"
sudo apt install snapd -y

echo && echo "Install virtualization"
sudo apt install virt-manager docker -y

echo && echo "Cargo build monorepo"
cargo build --release --manifest-path witch_oracle/src-tauri/Cargo.toml
cargo build --release --manifest-path witch_craft/Cargo.toml

echo && echo "Move applications to release"
mkdir -p ./release

cp -r ./witch_oracle/src-tauri/target/release/witch_oracle ./release/
cp -r ./witch_craft/target/release/witch_craft ./release/

echo && echo "TEST THE BINARY EXIT CODES"

chmod +x ./release/*
cd ./release

# Test and print status for each binary
test_binary() {
  ./$1
  status=$?
  if [ $status -eq 0 ]; then
    echo "$1 exited successfully."
  else
    echo "$1 exited with an error code."
  fi
}

test_binary "witch_craft"
test_binary "witch_oracle"
