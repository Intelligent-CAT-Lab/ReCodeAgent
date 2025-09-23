# AES Encryption 

A lightweight and optimized AES-128/256 encryption library written in C, designed for multi-uses cases. This library supports padding for arbitrary message lengths and includes optimizations leveraging SSE (Streaming SIMD Extensions) to enhance performance.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [License](#license)

## Features

- **AES-128/256 Implementation:** Provides encryption and decryption using the AES-128 standard (for aes256, you have to changes the main and the sizes).
- **Padding Support:** Implements PKCS#7 padding to handle messages of arbitrary lengths.
- **Key Expansion:** Efficient key expansion with SSE optimizations for faster performance.
- **SSE Optimizations:** Utilizes Streaming SIMD Extensions to accelerate critical cryptographic operations.
- **Easy Integration:** Designed to be easily integrated into existing IRC server architectures.
- **Standalone:** Does not rely on external cryptographic libraries, ensuring minimal dependencies.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/at0m741/aes256-SIMD.git
   cd aes256-SIMD
   make
   ```
