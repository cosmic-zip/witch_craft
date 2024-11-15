mkdir ./dist
cp ./witch_craft/target/release/witch_craft ./dist/
cp witch_spells ./dist
zip -r dist.zip dist/
mv dist.zip dist/
cd dist
zip witch_craft.zip witch_craft
