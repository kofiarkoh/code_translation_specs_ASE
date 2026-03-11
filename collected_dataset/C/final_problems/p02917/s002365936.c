#include <stdio.h>

int main(void){
	int N, cnt = 0;;
  	scanf("%d", &N);

  	int A[N], B[N-1];
  	for (int i = 0; i < N; i++) {
      	scanf("%d", &B[i]);
    }

	A[0] = B[0];
  	for (int i = 0; i < N - 2; i++) {
    	if(B[i] >= B[i + 1]) {
        	A[i + 1] = B[i + 1];
        }else{
        	A[i + 1] = B[i];
        }
    }

  	A[N - 1] = B[N - 2];
    for (int i = 0; i < N; i++) {
    	cnt += A[i];
    }

  	printf("%d\n", cnt);
}
