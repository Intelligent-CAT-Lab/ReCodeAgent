# C-ACES

## Overview
**C-ACES** is a C implementation of [ACES](https://arxiv.org/pdf/2401.13255.pdf) (Arithmetic Channel Encryption Scheme), a novel cryptosystem constructed using **category theory**, specifically leveraging the **Yoneda Lemma**. ACES eliminates the need for bootstrapping techniques, offering a more efficient and secure **Fully Homomorphic Encryption (FHE) scheme**.

This repository provides a practical and efficient implementation of ACES in C, enabling experimentation and integration into various systems requiring secure computations.
It serves as an educational and experimental endeavor for exploring the implementation of ACES.

Please note that a Python implementation of ACES is already available, For additional documentation please refer to the [original Python implementation repository](https://github.com/remytuyeras/aces). You can find extensive documentation and resources there to aid in understanding and using ACES.


## Getting Started
### Prerequisites                          
- CMake (version 3.16 or higher)                               
- A C compiler that supports C11 (e.g., GCC or Clang)         

### Building
1. Clone the Cring repository to your local machine:            
```bash   
   git clone https://github.com/your-username/Cring.git       
```
2. build
```bash
  cmake -B Release -DCRING_EXAMPLE=ON -DCRING_TEST=ON .
  cmake --build Release
```
### Examples
Here's a simple example of using C-ACES to do FHE operations:
```c
// initialize aces with proper parameters
Aces aces;
set_aces(&aces, DIM, memory, MEMORY_SIZE);
init_aces(2/*p*/, 33/*q*/, DIM, &aces);

// encrypt message 1
uint64_t message_1 = 4;
CipherMessage encrypted_message_1;
aces_encrypt(&aces, &message_1, 1, &encrypted_message_1);

// encrypt message 2
uint64_t message_2 = 3;
CipherMessage encrypted_message_2;
aces_encrypt(&aces, &message_1, 1, &encrypted_message_1);

uint64_t message_result;
CipherMessage encrypted_result;

// add encrypted_message_1 and encrypted_message_2
aces_add(&encrypted_message_1, &encrypted_message_2, &aces.shared_info, &encrypted_result);

// decrypt result
aces_decrypt(&aces, &encrypted_result, 1, &message_result);

uint64_t expected_result = (message_1 + message_2) % 2/*p*/;
assert(expected_result == message_result);
``` 
For more usage examples, explore the [examples](examples) folder.

## References

- Rémy Tuyéras. (2024). Constructing a fully homomorphic encryption scheme with the Yoneda Lemma. arXiv preprint arXiv:2401.13255. [Link](https://arxiv.org/abs/2401.13255)