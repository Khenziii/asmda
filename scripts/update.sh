#!/bin/sh

read -p "Which version would you like to use? (e.g.: v1.1.0): " version

echo "Downloading ASMDA $version..."

wget "https://github.com/Khenziii/asmda/releases/download/$1/asmda-$1-x86_64-unknown-linux-gnu" || { echo "Failed to run 'wget', exiting.."; exit; }

echo "Killing previous processes..."

pkill -9 asmda

echo "Setting up new binary..."

mv "asmda-$1-x86_64-unknown-linux-gnu" "asmda-unwrapped"
chmod +x asmda-unwrapped
sudo rm /usr/bin/asmda-unwrapped
sudo mv asmda-unwrapped /usr/bin

echo "Finished! You can now use \`asmda\` as usual:"
echo "\$ asmda --version"
asmda --version
