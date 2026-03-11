#include <stdio.h>

#define mMin(a, b) ((a) < (b))? (a): (b)

unsigned long min;		/* 10^4/1*10^4 = 10^8(27bit) */

int A[1000], B[1000];
unsigned long D[10001];

int main(void){
	int H, N, i, j;
	
	scanf("%d%d",&H, &N);
	
	for (i = 0; i < N; i++){
		scanf("%d%d", A + i, B + i);
	}
	
	/* ライフ1からHまで最小魔力でMonster倒すテーブルを作成 */
	for (i = 1; i <= H; i++){
		min = 0xFFFFFFFF;

		for (j = 0; j < N; j++){
			if (A[j] < i){
				min = mMin(D[i - A[j]] + B[j], min);
			}
			else{
				min = mMin(B[j], min);
			}
		}
		D[i] = min;
	}
	
	printf("%lu\n", D[H]);
	
	return 0;
}
