#!/bin/bash
set -e

echo && echo "Install dependencies"
#!/bin/bash

# Check if the package manager is apt or yum
if command -v apt &> /dev/null; then
    package_manager="apt"
elif command -v yum &> /dev/null; then
    package_manager="yum"
else
    echo "Unsupported package manager. Please install the packages manually."
    exit 1
fi

# Install packages
if [ "$package_manager" == "apt" ]; then
    sudo apt update
    sudo apt install -y nmap whois dirb dnsenum libc-bin iproute2 xxd iptables coreutils wget curl \
    dnsutils traceroute openssl openssh-server xattr libimage-exiftool-perl tor foremost pkg-config \
    libssl-dev steghide doas nala libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev \ 
    libayatana-appindicator3-dev librsvg2-dev
elif [ "$package_manager" == "yum" ]; then
    sudo yum update -y
    sudo yum install -y nmap dirb dnsenum glibc-utils xxd iptables-utils iproute wget curl bind-utils traceroute
fi

# Install data
echo && echo "Install witch_spells data"
sudo mkdir -p /var/witch_craft
sudo cp -r witch_spells/ /var/witch_craft
sudo chown -R $(whoami):$(whoami) /var/witch_craft

# Build binary
echo && echo "Cargo build"
cargo build --release --manifest-path witch_craft/Cargo.toml
chmod +x ./witch_craft/target/release/witch_craft
sudo cp -r ./witch_craft/target/release/witch_craft /bin


# Test and print status for each binary
witch_craft
if [ $? -eq 0 ]; then
    echo "Exit code is 0, all good!"
else
    echo "Exit code is not 0, something went wrong."
    exit
fi
