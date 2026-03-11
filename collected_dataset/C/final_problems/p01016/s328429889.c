#include <stdio.h>
#include <string.h>

char A[1024];
char B[1024];

size_t Alen, Blen;

int main(void) {
	size_t i, j;
	if (scanf("%1023s", A) != 1) return 1;
	if (scanf("%1023s", B) != 1) return 1;
	Alen = strlen(A);
	Blen = strlen(B);
	for (i = 0; i <= Alen - Blen; i++) {
		int ok = 1;
		for (j = 0; j < Blen; j++) {
			if (B[j] != '_' && A[i + j] != B[j]) {
				ok = 0;
				break;
			}
		}
		if (ok) {
			puts("Yes");
			return 0;
		}
	}
	puts("No");
	return 0;
}

