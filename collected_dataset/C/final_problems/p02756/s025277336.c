#include <stdio.h>
#include <string.h>

int main() {
	char S[200001], C, tmp;
	unsigned int Q, i, j, x;
	unsigned short T, F;

	memset(S, 0, sizeof(S));
	scanf("%s", S);
	scanf("%u", &Q);
	for (i = 0; i < Q; i++) {
		scanf("%hu", &T);
		if (T == 1) {
			x = 0;
			for (j = strlen(S) - 1; strlen(S) / 2 <= j; j--) {
				tmp  = S[j];
				S[j] = S[x];
				S[x] = tmp;
				x++;
			}
		} else {
			scanf("%hu %c", &F, &C);
			if (F == 1) {
				memmove(&S[1], &S[0], strlen(S));
				S[0] = C;
			} else {
				S[strlen(S)] = C;
			}
		}
	}
	printf("%s\n", S);
	return 0;
}
