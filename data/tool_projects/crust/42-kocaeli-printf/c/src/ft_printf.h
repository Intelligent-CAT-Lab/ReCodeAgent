/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_printf.h                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: eates <eates@student.42kocaeli.com.tr>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/09/05 13:05:07 by eates             #+#    #+#             */
/*   Updated: 2023/09/05 13:07:09 by eates            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef FT_PRINTF_H
# define FT_PRINTF_H
# if defined(__APPLE__)
#  define LOCATION 1
# elif defined(__linux__)
#  define LOCATION 2
# endif
# include <stdarg.h>
# include <stdint.h>

# define DECIMAL "0123456789"
# define HEXALOW "0123456789abcdef"
# define HEXAUP "0123456789ABCDEF"

int		writechar(char c, int *len);
int		writestring(char *s, int *len);
int		writeint(int n, int *len);
int		writeuint(unsigned long n, int *len);
int		writepoint(void *n, int *len);
int		writehex(unsigned long n, char c, int *len);
int		ft_printf(const char *s, ...);

#endif
