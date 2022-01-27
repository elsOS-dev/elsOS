#include <stdint.h>
#include <stddef.h>
#include "libc.h"

void	*memset(void *b, int c, size_t len)
{
	size_t		i;

	i = 0;
	while (i < len)
	{
		((char*)b)[i] = c;
		i++;
	}
	return (b);
}
