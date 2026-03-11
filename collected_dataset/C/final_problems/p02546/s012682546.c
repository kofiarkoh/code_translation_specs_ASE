#include <stdio.h>

int main(void) {
	char S[1020];
	int N;
	scanf("%s", S);
	for (int i = 0; S[i] != '\0'; i++) {
		N = i;
	}
	if (S[N] == 's')
		printf("%ses", S);
	else
		printf("%ss", S);
	return 0;
}