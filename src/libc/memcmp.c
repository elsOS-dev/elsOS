#include <stdint.h>
#include <stddef.h>
#include "libc.h"

int	memcmp(const void *s1, const void *s2, size_t n)
{
	size_t i;

	i = 0;
	if (!n)
		return (0);
	while (i < n - 1 && ((unsigned char*)s1)[i] == ((unsigned char*)s2)[i])
		i++;
	return (((unsigned char*)s1)[i] - ((unsigned char*)s2)[i]);
}
