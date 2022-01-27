#include <stdint.h>
#include <stddef.h>
#include "libc.h"

void	*memmove(void *dst, const void *src, size_t len)
{
	size_t		i;

	i = 0;
	if (dst < src)
	{
		while (i < len)
		{
			((char*)dst)[i] = ((char*)src)[i];
			i++;
		}
	}
	else
	{
		while (len--)
		{
			((char*)dst)[len] = ((char*)src)[len];
		}
	}
	return (dst);
}
