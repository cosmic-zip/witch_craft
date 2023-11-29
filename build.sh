echo "MR Build System"

echo && echo "Install dependencies"
sudo apt install aptitude
sudo aptitude update -y
sudo aptitude install -y p7zip-full nmap curl xxd libc6 exiftool traceroute wget iproute2 whois dnsutils dirb dnsenum tree htop iftop clang 
sudo aptitude install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

echo && echo "Move config folders to /var"
sudo mkdir /var/maid
sudo cp maid_lists/ /var/maid -R
sudo chown $(whoami):$(whoami) /var/maid -R

echo && echo  "Uncompress files"
7z x /var/maid/maid_lists/malware/malware.csv.7z.001
mv ./full.csv /var/maid/maid_lists/general/malware_hash.bin

echo && echo "SNAP Setup"
sudo aptitude install snapd -y
sudo snap install vlc -y

echo && echo "Install virtualization"
sudo aptitude install virt-manager docker -y

echo && echo "Cargo build monorepo"
cargo build --release --manifest-path maid_visual/src-tauri/Cargo.toml
cargo build --release --manifest-path maid_runner/Cargo.toml
cargo build --release --manifest-path maid_api/Cargo.toml

echo && echo "Move applications to release"
mkdir ./release
yes | cp -r ./maid_visual/src-tauri/target/release/maid_visual  ./release/
yes | cp -r ./maid_runner/target/release/maid_runner ./release/
yes | cp -r ./maid_api/target/release/maid_api ./release/

echo && echo "DONE"