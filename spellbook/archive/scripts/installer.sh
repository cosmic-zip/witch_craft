#!/bin/bash
set -e
echo && echo "Installer"
sudo cp -nr spellbook/ /var/
sudo chown -R $(whoami):$(whoami) /var/spellbook
echo 'export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/
sudo cp -r witchcraft /bin/witchcraft
