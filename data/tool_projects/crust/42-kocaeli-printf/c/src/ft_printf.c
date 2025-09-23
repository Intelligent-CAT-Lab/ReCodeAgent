/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_printf.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: eates <eates@student.42kocaeli.com.tr>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/09/05 12:16:43 by eates             #+#    #+#             */
/*   Updated: 2023/09/05 12:16:44 by eates            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "ft_printf.h"

static int	format(va_list *args, char c, int *len)
{
	if (c == 'c')
		return (writechar(va_arg(*args, int), len));
	if (c == 's')
		return (writestring(va_arg(*args, char *), len));
	if (c == 'd' || c == 'i')
		return (writeint(va_arg(*args, int), len));
	if (c == 'u')
		return (writeuint(va_arg(*args, unsigned int), len));
	if (c == 'p')
		return (writepoint(va_arg(*args, void *), len));
	if (c == 'x' || c == 'X')
		return (writehex(va_arg(*args, unsigned int), c, len));
	if (c == '%')
		return (writechar('%', len));
	return (-1);
}

int	ft_printf(const char *s, ...)
{
	va_list		args;
	int			len;

	if (LOCATION == 2 && !s)
		return (-1);
	len = 0;
	va_start(args, s);
	while (*s)
	{
		if (*s == '%')
		{
			if (format(&args, *++s, &len) == -1)
				return (va_end(args), -1);
		}
		else if (writechar(*s, &len) == -1)
			return (va_end(args), -1);
		s++;
	}
	return (va_end(args), len);
}
int	writeint(int n, int *len)
{
	char	arr[10];
	int		i;

	i = 0;
	if (!n)
		return (writechar('0', len));
	if (n == -2147483648)
		return (writestring("-2147483648", len));
	if (n < 0)
	{
		if (writechar('-', len) == -1)
			return (-1);
		n = -n;
	}
	while (n)
	{
		arr[i++] = DECIMAL[n % 10];
		n /= 10;
	}
	while (i--)
		if (writechar(arr[i], len) == -1)
			return (-1);
	return (1);
}

int	writeuint(unsigned long n, int *len)
{
	int		arr[16];
	int		i;

	i = 0;
	if (!n)
		return (writechar('0', len));
	while (n)
	{
		arr[i++] = DECIMAL[n % 10];
		n /= 10;
	}
	while (i--)
		if (writechar(arr[i], len) == -1)
			return (-1);
	return (1);
}

int	writehex(unsigned long n, char c, int *len)
{
	char	arr[16];
	int		i;
	char	*hex;

	i = 0;
	if (!n)
		arr[i++] = '0';
	if (c == 'x')
		hex = HEXALOW;
	else
		hex = HEXAUP;
	while (n)
	{
		arr[i++] = hex[n % 16];
		n /= 16;
	}
	while (i--)
		if (writechar(arr[i], len) == -1)
			return (-1);
	return (1);
}

int	writepoint(void *ptr, int *len)
{
	char				arr[32];
	int					i;
	unsigned long long	nb;

	if (LOCATION == 2 && !ptr)
		return (writestring("(nil)", len));
	i = 0;
	nb = (unsigned long long)ptr;
	if (writestring("0x", len) == -1)
		return (-1);
	if (!nb)
		return (writechar('0', len));
	while (nb)
	{
		arr[i++] = HEXALOW[nb % 16];
		nb /= 16;
	}
	while (i--)
		if (writechar(arr[i], len) == -1)
			return (-1);
	return (1);
}
int	writechar(char c, int *len)
{
	return ((*len)++, write(1, &c, 1));
}

int	writestring(char *s, int *len)
{
	if (!s)
		s = "(null)";
	while (*s)
		if (writechar(*s++, len) == -1)
			return (-1);
	return (1);
}