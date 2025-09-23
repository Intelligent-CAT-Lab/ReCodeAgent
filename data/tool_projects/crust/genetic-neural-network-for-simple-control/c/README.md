# Genetic-neural-network-for-simple-control\

## Overview
This project was created to create Neural Network models with much faster calculations than in C. 

The main purpose of this model is to be used in simple control tasks replacing the PID, which can't normally control complex non-linear systems. This control in practice can be used on robotics and control systems such as water level control, air pollution control etc.

## Models
Status of the models' types:
- FF - working 
- SD - in testing  
- RR - planned

## Activation functions
As activation functions following functions are used:
1) Sigmoids ![equation](https://latex.codecogs.com/svg.image?%5Csigma(x)=%5Cfrac%7B1%7D%7B1&plus;e%5E%7B-2%7D%7D,%5Cquad%5Csigma%5Cin%3C0,1%3E)
2) Tangenth ![equation](https://latex.codecogs.com/svg.image?%5Ctanh(x)=%5Cfrac%7Be%5E%7Bx%7D-e%5E%7B-x%7D%7D%7Be%5E%7Bx%7D&plus;e%5E%7B-x%7D%7D,%5Cquad%5Csigma%5Cin%3C-1,1%3E)

Tangenth activation function is preferred due to better range.

## Testing
### Code testing
Currently, the tests are written in the folder TEST with test_main being used as a linking file to other test files. Each of the specific test files includes methods to test different functions of the parts of the system. The tests should be run with the following terminal prompt:
```shell
valgrind --exit-on-first-error=yes --leak-check=full ./testprogram
```
As well as following combination of commands can provide valid data for the function time running and performance:
```shell
./testprogram
gprof testprogram
```
Also, the following line can be used to get the number of lines of code used in the project:
```shell
# to find number of rows in project
find . -type f \( -name "*.c" -o -name "*.h" \) -exec wc -l {} +
# to find number of symbols in project
find . -type f \( -name "*.c" -o -name "*.h" \) -exec cat {} + | wc -c
```
Also, if the WSL is used before the commit following command should be executed to allow the terminal GPG key
```shell
GPG_TTY=$(tty)
export GPG_TTY
```