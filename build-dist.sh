echo && echo "Build packages"
mkdir ./dist

cp ./witch_craft/target/release/witch_craft ./dist/
cp witch_spells ./dist

zip -r witch_craft-full.zip dist/
mv witch_craft-full.zip dist/

rm -r dist/witch_spells/archive/wordlists/
rm -r dist/witch_spells/archive/malware/
rm  dist/witch_spells/archive/osint/Archive.7z.00*
zip -r witch_craft-lite.zip dist/
mv witch_craft-lite.zip dist/

rm -r dist/witch_spells
rm witch_craft
rm install.sh
