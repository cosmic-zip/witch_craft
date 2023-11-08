#! /bin/bash

echo && echo  "Uncompress files"
7z x /var/maid/maid_lists/malware/full.csv.7z.001
mv full.csv /var/maid/maid_lists/general/malware_hash.bin