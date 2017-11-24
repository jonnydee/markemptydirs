#/bin/sh

pushd test
chmod a+rwx private
chmod a+rwx ro-without-marker
chmod a+rwx ro-with-marker
popd

rm -rf ./test/
mkdir test
cd test

mkdir -p a/b/c a/b/d
mkdir -p e
mkdir -p j/k/l
mkdir private
mkdir ro-without-marker
mkdir ro-with-marker

ln -s j/k/l link-to-jkl

pushd e
ln -s ../a/b link-to-ab
popd

touch .emptydir
touch a/.emptydir
touch a/b/.emptydir
touch a/b/file_1
touch a/b/c/.emptydir
touch private/.emptydir
touch ro-with-marker/.emptydir
touch ro-with-marker/file_2

chmod a-rwx private
chmod a-w ro-without-marker
chmod a-w ro-with-marker

