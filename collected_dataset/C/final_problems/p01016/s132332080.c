// AOJ 1526: Password
// 2019.2.20 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char A[1005], B[1005];

int match(char *a, int la, char *b, int lb)
{
	int i, j, w;
	
	w = la - lb;
	for (i = 0; i <= w; i++) {
		for (j = 0; ; j++) {
			if (j == lb) return 1;
			if (b[j] != '_' && a[i + j] != b[j]) break;
		}
	}
	return 0;
}

int main()
{
	scanf("%s%s", A, B);
	puts(match(A, strlen(A), B, strlen(B)) ? "Yes" : "No");
	return 0;
}

