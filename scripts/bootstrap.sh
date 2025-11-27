#!/bin/bash

startup_warning () {
	echo "WARNING: This script was created to download and setup the ASMDA application."

	echo ""

	echo "It will download a binary and some scripts from https://github.com/khenziii/asmda"
	echo "Using them, you'll later be able to:"
	echo "- switch ASMDA's versions,"
	echo "- launch the app within an environment with defined settings,"
	echo "- etc."
    echo "Think of it like a minimalistic launcher setup."

	echo ""

	read -p "Do you want to proceed? (Y/n): " response

	# `^^` converts the string to uppercase
	if [ "${response^^}" != "Y" ]; then
		echo "Exiting.."
		exit
	fi

	echo "Proceeding.."
}

# Exits, if the script doesn't have necessary permissions
check_if_executed_by_root() {
	if ! [ "$EUID" = "0" ]; then
    	echo "This script needs to be executed with 'sudo', or by the root user. Exiting.."
		exit
	fi
}

# Exits, if $1 is not installed
check_if_installed () {
	if ! which $1 > /dev/null 2>&1; then
		echo "$1 is not installed. Please install it, and rerun the script."
		exit
	fi	
}

agreement=$1
if [ "${agreement^^}" != "Y" ]; then
	startup_warning
fi

check_if_executed_by_root
check_if_installed "wget"

echo "Fetching the scripts..."

wget https://raw.githubusercontent.com/khenziii/asmda/master/scripts/update.sh || { echo "Failed to run 'wget', exiting.."; exit; }
wget https://raw.githubusercontent.com/khenziii/asmda/master/scripts/run.sh || { echo "Failed to run 'wget', exiting.."; exit; }

echo "Setting permissions..."

chmod +x update.sh
chmod +x run.sh

echo "Copying scripts..."

mv update.sh asmda-update && sudo mv asmda-update /usr/local/bin
mv run.sh asmda && sudo mv asmda /usr/local/bin

echo "Success! Everything has been setup correctly. You'll now have to choose the version of ASMDA to install."
echo "If you'd like to switch it in the future, you can rerun \`asmda-update\` and choose a different one without any issues."

echo ""

sudo asmda-update
