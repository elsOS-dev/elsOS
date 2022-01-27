#include <stdint.h>
#include <stddef.h>
#include "libc.h"
void	*memcpy(void *dst, const void *src, size_t n)
{
	size_t		i;

	i = 0;
	while (i < n)
	{
		((char*)dst)[i] = ((char*)src)[i];
		i++;
	}
	return (dst);
}
