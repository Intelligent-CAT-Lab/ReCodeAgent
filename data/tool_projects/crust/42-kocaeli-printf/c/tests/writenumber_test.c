/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   writenumber.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: eates <eates@student.42kocaeli.com.tr>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/09/05 12:16:20 by eates             #+#    #+#             */
/*   Updated: 2023/09/05 12:16:23 by eates            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "../src/ft_printf.h"
#include <assert.h>
#include <stdio.h>



int main(){
	 int len = 0;

    // Test writeint function
    len = 0;
    assert(writeint(42, &len) == 1);   // Test with a positive number
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writeint(-42, &len) == 1);  // Test with a negative number
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writeint(0, &len) == 1);    // Test with zero
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writeint(-2147483648, &len) == 1); // Test with the minimum integer value
    assert(len > 0);                   // Length should increase after writing

    // Test writeuint function
    len = 0;
    assert(writeuint(42, &len) == 1);  // Test with a positive unsigned number
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writeuint(0, &len) == 1);   // Test with zero
    assert(len > 0);                   // Length should increase after writing

    // Test writehex function
    len = 0;
    assert(writehex(42, 'x', &len) == 1); // Test with a positive number in lowercase hexadecimal
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writehex(42, 'X', &len) == 1); // Test with a positive number in uppercase hexadecimal
    assert(len > 0);                   // Length should increase after writing

    len = 0;
    assert(writehex(0, 'x', &len) == 1);  // Test with zero in hexadecimal
    assert(len > 0);                    // Length should increase after writing

    // Test writepoint function
    len = 0;
    void *ptr = (void*)0x1234;  // A dummy pointer value
    assert(writepoint(ptr, &len) == 1);  // Test with a valid pointer
    assert(len > 0);                    // Length should increase after writing

    len = 0;
    ptr = NULL;  // Null pointer
    assert(writepoint(ptr, &len) == 1);  // Test with NULL pointer
    assert(len > 0);                    // Length should increase after writing

    printf("All tests passed successfully.\n");
    return 0;
}