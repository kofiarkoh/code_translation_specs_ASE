#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, Q, i;
	scanf("%d", &N);
	int *c = (int *)malloc(sizeof(int) * N);
	int *now_c = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &c[i]);
		now_c[i] = 0;
	}
	scanf("%d", &Q);
	int *t = (int *)malloc(sizeof(int) * Q);
	int *x = (int *)malloc(sizeof(int) * Q);
	int *d = (int *)malloc(sizeof(int) * Q);
	for(i = 0; i < Q; i++){
		scanf("%d%d%d", &t[i], &x[i], &d[i]);
		x[i]--;
	}
	for(i = 0; i < Q; i++){
		if(t[i] == 1){
			now_c[x[i]] += d[i];
			if(now_c[x[i]] > c[x[i]]){
				printf("%d\n", x[i] + 1);
				return 0;
			}
		}
		else{
			now_c[x[i]] -= d[i];
			if(now_c[x[i]] < 0){
				printf("%d\n", x[i] + 1);
				return 0;
			}
		}
	}
	printf("0\n");
	return 0;
}
