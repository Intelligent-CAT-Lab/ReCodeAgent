#!/bin/bash

# Remove the CLI tool executable
if [ -f "/usr/local/bin/vaultsync" ]; then
    sudo rm /usr/local/bin/vaultsync
    echo "CLI tool 'vaultsync' has been uninstalled."
else
    echo "CLI tool 'vaultsync' is not installed."
fi

# Remove the manual page
if [ -f "/usr/share/man/man1/vaultsync.1" ]; then
    sudo rm /usr/share/man/man1/vaultsync.1
    sudo mandb  # Update Manpage Database
    echo "Manual page 'vaultsync.1' has been uninstalled."
else
    echo "Manual page 'vaultsync.1' is not installed."
fi

