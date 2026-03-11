#include <stdio.h>
#include <stdlib.h>

typedef enum{
	win,
	lose
}res;

res judge(int *A, int N){
	int i, evennum = 0, oddnum = 0, odd, one = 0;
	for(i = 0; i < N; i++){
		if(A[i] % 2 == 0){
			evennum++;
		}
		else{
			if(A[i] == 1){
				one++;
			}
			oddnum++;
			odd = i;
		}
	}
	if(evennum == N){
		for(i = 0; i < N; i++){
			A[i] /= 2;
			return judge(A, N);
		}
	}
	else if(evennum % 2 == 1){
		return lose;
	}
	else if(evennum < N - 1 || one != 0){
		return win;
	}
	else{
		A[odd]--;
		return !judge(A, N);
	}
}

int main(){
	int N, i;
	scanf("%d", &N);
	int *A = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%d", &A[i]);
	}
	if(judge(A, N) == win){
		printf("Second\n");
	}
	else{
		printf("First\n");
	}
	return 0;
}