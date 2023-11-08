#!/bin/bash
echo && echo  "Uncompress files"
7z x /var/maid/maid_lists/malware/full.csv.7z.001
mv /var/maid/maid_lists/malware/full.csv /var/maid/maid_lists/malware/malware_hash.bin
mv /var/maid/maid_lists/malware/malware_hash.bin /var/maid/maid_lists/general/