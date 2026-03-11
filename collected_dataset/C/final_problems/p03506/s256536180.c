#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, Q, i;
	scanf("%d%d", &N, &Q);
	int *v = (int *)malloc(sizeof(int) * Q);
	int *w = (int *)malloc(sizeof(int) * Q);
	for(i = 0; i < Q; i++){
		scanf("%d%d", &v[i], &w[i]);
	}
	if(N == 1){
		for(i = 0; i < Q; i++){
			printf("%d\n", v[i]);
		}
	}
	else{
		for(i = 0; i < Q; i++){
			while(v[i] != w[i]){
				if(v[i] < w[i]){
					w[i] = (w[i] + N - 2) / N;
				}
				else{
					v[i] = (v[i] + N - 2) / N;
				}
			}
			printf("%d\n", v[i]);
		}
	}
	return 0;
}