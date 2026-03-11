#include <stdio.h>
#include <stdlib.h>

int main()
{
	char c;
	scanf("%c", &c);

	if(c == 'A'){
		printf("T\n");
	} else if(c == 'T'){
		printf("A\n");
	} else if(c == 'G'){
		printf("C\n");
	} else if(c == 'C'){
		printf("G\n");
	} else {
		abort();
	}
}
