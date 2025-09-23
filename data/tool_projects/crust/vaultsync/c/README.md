## Version Tracking and Rollback

This project offers robust version tracking capabilities, ensuring that every change made to your files is carefully monitored and logged. With our tool, you can rest assured that you'll never lose track of your file versions, even in the most dynamic collaborative environments.

### Key Features:

- **Comprehensive Version tracking**: Every modification made to your files you can record it.
  
- **Effortless Rollback**: Need to revert to a previous version? No problem! Our tool allows users to effortlessly roll back to any desired version with one command.
  
- **Intuitive Command Line Interface**: Initialization, tracking of changes, record them and rollbacks are made simple and intuitive, ensuring a seamless user experience.

### Supported Operations:

- **Track Changes**: Whether it's a minor tweak or a major overhaul, our tool tracks every change made to your files, including additions, modifications, and deletions.
  
- **Restore Previous Versions**: Accidentally overwritten an important file or made changes you now regret? Our rollback feature lets you restore previous versions with ease, giving you peace of mind knowing that your data is always recoverable.

### Installing:
To install this project ....
```bash
	$ git clone git@github.com:elhalili/VaultSync.git
	$ cd VaultSync
	$ chmod +x scripts/setup.sh
	$ ./scripts/setup.sh
```

### Usage:

To take advantage of the version tracking and rollback features, simply follow these steps:
1. Initialize the repository using the provided CLI.
```bash
	# Change the cwd to the desired directory project
	cd path/to/your-project
	# Initialize the project
	vaultsync init
```
2. Make changes to your files as needed, knowing that every modification is being tracked in the background.
```bash
	# Track changes using
	vaultsync add [files-paths-to-track]
```
3. Register stable changes via commit :
```bash
	# Commit changes
	vaultsync commit
```
4. If you ever need to revert to a previous version, use the rollback command to select the desired version and restore it.
```bash
	# Rollback to any version 
	vaultsync rollback version-hash
```

With our version tracking and rollback capabilities, you can collaborate with confidence, knowing that your files are always protected and easily recoverable.
- [ ] Remote host (in progress)
- [ ] User Authentication (in progress)
- [ ] File Encryption and Decryption (in progress)
- [ ] Secure Communication (in progress)
- [ ] File Synchronization
- [ ] Conflict Resolution

