#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main()
{
	int N;
	scanf("%d", &N);
	int i, j, k;
	char S[N];
	for (i = 0; i < N; i++) 
		scanf("%c", &S[i]);

	int yellow = 0;
	for (i = 0; i < N; i++) {
		if (S[i] == 'Y')
			yellow = 1;
	}

/*	if (yellow)
		printf("Four\n");
	else
		printf("Three\n");
*/
	printf(yellow ? "Four\n" : "Three\n");

	return 0;
}
