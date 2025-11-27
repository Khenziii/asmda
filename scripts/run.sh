#!/bin/sh

if [ -e "$HOME/asmda-secrets.env" ]; then
    env $(grep -v '^#' $HOME/asmda-secrets.env | xargs) asmda-unwrapped "$@"
else
    echo "\`$HOME/asmda-secrets.env\` doesn't exist! Please either create it and specify the configuration there (for a quickstart copy one from the documentation), or use \`asmda-unwrapped\` to manually specify the environment variables, without automatically loading them from this file."
fi
