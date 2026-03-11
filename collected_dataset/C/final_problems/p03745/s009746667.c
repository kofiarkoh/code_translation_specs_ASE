#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, i, ans = 1;
	scanf("%d", &N);
	int *A = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &A[i]);
	}
	int now = 0;
	for(i = 1; i < N; i++){
		if(now == 0){
			if(A[i] > A[i - 1]){
				now = 1;
			}
			else if(A[i] < A[i - 1]){
				now = -1;
			}
		}
		else if(now == 1){
			if(A[i] < A[i - 1]){
				ans++;
				now = 0;
			}
		}
		else{
			if(A[i] > A[i - 1]){
				ans++;
				now = 0;
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}