# Install data
echo && echo "Install witch_spells data"
sudo cp -nr witch_spells/ /var/
sudo chown -R $(whoami):$(whoami) /var/witch_spells
7z x /var/witch_spells/archive/osint/Archive.7z.001 -o/var/witch_spells/archive/osint/
7z x /var/witch_spells/archive/malware/full.7z.001 -o/var/witch_spells/archive/malware/
echo 'export WITCH_SPELLS_ROOT_DIR=/var/witch_spells/archive/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/witch_spells/archive/

# Wordlist
echo && echo "Wordlists are big (16GB) so, if you want or need then, just run:"
echo "7z x /var/witch_craft/witch_spells/wordlists/ladybug.pwned"
echo "7z x /var/witch_craft/witch_spells/wordlists/moth.pwned.7z.001"

# Install inside /bin
sudo cp -r ./witch_craft /bin/witchcraft
sudo ln -s /bin/witchcraft /bin/witchy
