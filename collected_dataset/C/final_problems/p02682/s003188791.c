#include<stdio.h>

int main(){
	long int A, B, C, K;
	scanf("%ld %ld %ld %ld", &A, &B, &C, &K);
	if(K <= A)
		printf("%ld", K);
	else if(K <= A + B)
		printf("%ld", A);
	else
		printf("%ld", 2 * A + B - K);
	return 0;
}