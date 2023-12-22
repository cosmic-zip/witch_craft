#!/bin/bash
set -e

echo "MR Build System"

echo && echo "Install dependencies"
sudo apt update -y
sudo apt install -y aptitude p7zip-full nmap curl xxd libc6 exiftool traceroute wget iproute2 whois dnsutils dirb dnsenum tree htop iftop clang sudo libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

echo && echo "Move config folders to /var"
sudo mkdir -p /var/maid
sudo cp -r maid_lists/ /var/maid
sudo chown -R $(whoami):$(whoami) /var/maid

echo && echo "Uncompress files"
7z x /var/maid/maid_lists/malware/malware.csv.7z.001 -o/var/maid/maid_lists/general/
mv /var/maid/maid_lists/general/full.csv /var/maid/maid_lists/general/malware_hash.config

echo && echo "SNAP Setup"
sudo apt install snapd -y
sudo snap install vlc

echo && echo "Install virtualization"
sudo apt install virt-manager docker -y

echo && echo "Cargo build monorepo"
cargo build --release --manifest-path maid_visual/src-tauri/Cargo.toml
cargo build --release --manifest-path maid_runner/Cargo.toml
cargo build --release --manifest-path maid_api/Cargo.toml

echo && echo "Move applications to release"
mkdir -p ./release
cp -r ./maid_visual/src-tauri/target/release/maid_visual ./release/
cp -r ./maid_runner/target/release/maid_runner ./release/
cp -r ./maid_api/target/release/maid_api ./release/

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

test_binary "maid_api"
test_binary "maid_runner"
test_binary "maid_visual"
