#ifndef LIBC_H
# define LIBC_H

# include <stddef.h>

void	*memset(void *b, int c, size_t len);
void	*memcpy(void *dst, const void *src, size_t n);
void	*memmove(void *dst, const void *src, size_t len);
int		memcmp(const void *s1, const void *s2, size_t n);
size_t	strlen(const char *s1);

#endif
