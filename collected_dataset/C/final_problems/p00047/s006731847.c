#include <stdio.h>

int main()
{
	char c1, c2, pos;

	pos = 'A';

	while(scanf("%c,%c", &c1, &c2) != EOF){
		if(c1 == pos) c1 = pos;
		else if(c2 == pos) c2 = pos;
	}

	printf("%c\n", pos);

	return 0;
}