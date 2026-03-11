#include <stdio.h>

#define N 50

int main(void) {
	int n,r;
	int c[N];
	int deck_A[N],deck_B[N],deck_C[N];
	int i,j,k;
	int len;
	int last;

	// input the number of cards n and the number of operations r
	while (scanf("%d%d",&n,&r) != EOF) {
		// input each operation c
		for (i = 0;i != r;++i) {
			scanf("%d",c + i);
		}
		// init the deck
		for (i = 0;i != n;++i) {
			deck_C[i] = i;
		}
		// riffle shuffle
		for (i = 0;i != r;++i) {
			// divide cards into deck A and deck B
			// deck A consists of the top half, and deck B of the bottom half
			len = n/2;
			for (j = 0;j != len;++j) {
				deck_B[j] = deck_C[j];
			}
			for (k = 0;j != n;++k,++j) {
				deck_A[k] = deck_C[j];
			}
			// operate
			len = n/2 - n/2%c[i];
			// j is index of deck A and B,k is index of deck C
			for (j = 0,k = 0;j != len;++j) {
				deck_C[k] = deck_A[j];
				deck_C[k + c[i]] = deck_B[j];
				k += (j + 1)%c[i] == 0 ? 1 + c[i] : 1;
			}
			len = 1 + n/2;
			last = j;
			for (;j != len;++j,++k) {
				deck_C[k] = deck_A[j];
			}
			len = n/2;
			for (j = last;j != len;++j,++k) {
				deck_C[k] = deck_B[j];
			}
		}
		printf("%d\n",deck_C[n - 1]);
	}
	return 0;
}