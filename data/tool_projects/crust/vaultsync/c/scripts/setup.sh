#!/bin/bash

# Define the environment variable for the configuration file path
export VSYNC_CONFIG_PATH=~/.vsync_rc

# Remove existing installation if present
if [ -f "/usr/local/bin/vaultsync" ]; then
    sudo rm /usr/local/bin/vaultsync
fi

# Move back to the project root directory
cd ..

# Check if build directory exists, if not, create it
if [ ! -d "build" ]; then
    mkdir -p build
fi

# Build the C project CLI
cd build
cmake ..
make

# Copy the executable to a location in PATH
sudo cp vaultsync /usr/local/bin

# Create the manual page
cat > vaultsync.1 << 'EOF'
.\" Manpage for vaultsync
.TH vaultsync 1 "February 2024" "Version 1.0" "vaultsync Manual"

.SH NAME
vaultsync \- CLI tool for managing repositories

.SH SYNOPSIS
vaultsync [\fIOPTION\fR]... [\fICOMMAND\fR]...

.SH DESCRIPTION
Vaultsync is a command line tool for managing repositories.

.SH OPTIONS
\fB\-h, \-\-help\fR
    Show help message and exit.

.SH COMMANDS
\fBinit\fR
    Initialize a repository.

\fBcommit\fR
    Make a commit.

\fBadd\fR \fI[files]\fR
    Add files to be tracked in the next commit.

.SH AUTHOR
Written by Your Amine EL HALILI.

.SH SEE ALSO
vaultsync.conf(5)

.SH BUGS
Report bugs to: amine.elhalili1@gmail.com
EOF

# Install the manual page
if [ -f "/usr/share/man/man1/vaultsync.1" ]; then
    sudo rm /usr/share/man/man1/vaultsync.1
fi
sudo cp vaultsync.1 /usr/share/man/man1/

# Update Manpage Database
sudo mandb

# Prompt for username
read -p "Enter your username: " username

# Prompt for email
read -p "Enter your email: " email

# Create the configuration file with username and email
echo "$username $email" > "$VSYNC_CONFIG_PATH"

echo "Configuration file created at $VSYNC_CONFIG_PATH with the following content:"
echo "$username $email"

# Check if .bashrc exists and add the source line if so
if [ -f ~/.bashrc ]; then
    echo -e "\n# Source VSYNC configuration script" >> ~/.bashrc
    echo "export VSYNC_CONFIG_PATH=~/.vsync_rc" >> ~/.bashrc
fi

# Check if .zshrc exists and add the source line if so
if [ -f ~/.zshrc ]; then
    echo -e "\n# Source VSYNC configuration script" >> ~/.zshrc
    echo "export VSYNC_CONFIG_PATH=~/.vsync_rc" >> ~/.zshrc
fi

echo "Setup complete. Restart your shell or run 'source ~/.bashrc' or 'source ~/.zshrc'."

