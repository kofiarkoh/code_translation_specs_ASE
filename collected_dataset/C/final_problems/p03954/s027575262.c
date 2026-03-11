#include <stdio.h>
#include <stdlib.h>

int median(int a, int b, int c){
	if (a < b)
		if(b <= c)
			return b;
		else
			if(a <= c)
				return c;
			else
				return a;
	else
		if(b <= c)
			if(a <= c)
				return a;
			else
				return c;
		else
			return b;

}

int main(){
	int N;
	int i,j;
	int *A, *A_temp;
	
	
	scanf("%d", &N);

	A = (int*)malloc(sizeof(int) * (2*N -1));
	A_temp = (int*)malloc(sizeof(int) * (2*N - 1));
	for(i=0; i<2*N-1; i++){
		scanf("%d", &A[i]); 
		A_temp[i] = 0;
	}

	for(i=0; i<N; i++){
		for(j=i; j < 2*N - 1 - i; j++){
			A_temp[j] = median(A[j-1], A[j], A[j+1]);
		}
		for(j=i; j < 2*N - 1 - i; j++){
			A[j] = A_temp[j];
		}

		
		
	}

	printf("%d\n", A[N-1]);




	return 0;
}
