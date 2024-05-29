#!/bin/bash
set -e

echo "Build System"

echo && echo "Install dependencies"
sudo apt update -y
sudo apt install -y aptitude p7zip-full nmap curl xxd libc6 exiftool \
traceroute wget iproute2 whois dnsutils dirb dnsenum tree htop iftop \
clang sudo build-essential curl wget file libssl-dev

echo && echo "Move config folders to /var"
sudo mkdir -p /var/witch_craft
sudo cp -r witch_spells/ /var/witch_craft
sudo cp -r blood_moon/ /var/witch_craft/witch_spells/private/
sudo chown -R $(whoami):$(whoami) /var/witch_craft

echo && echo "Cargo build monorepo"
cargo build --release --manifest-path witch_craft/Cargo.toml

echo && echo "Move applications to release"
mkdir -p ./release

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
