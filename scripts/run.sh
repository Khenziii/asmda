#!/bin/sh

if [ -e "$HOME/asmda-secrets.env" ]; then
    dotenv -e $HOME/asmda-secrets.env asmda-unwrapped "$@"
else
    echo "\`$HOME/asmda-secrets.env\` doesn't exist! Please either create it and specify the configuration there (for a quickstart copy one from the documentation), or use \`asmda-unwrapped\` to manually specify the environment variables, without automatically loading them from this file."
fi
