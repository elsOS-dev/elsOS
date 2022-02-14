NAME=libc.a
SRC=memset.c memmove.c memcpy.c memcmp.c
OBJ=${SRC:.c=.o}
ARCH=i686
AR=$(ARCH)-elf-ar
ARFLAGS=rcs

all: $(NAME)

$(NAME): $(OBJ)
	@$(AR) $(ARFLAGS) $(NAME) $(OBJ)

%.o: %.c
	@$(CC) $(CFLAGS) -o $@ $<

clean:
	@rm -f $(OBJ) $(NAME)
