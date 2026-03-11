#include<stdio.h>



#define aaa 1000000007

int main(void)
{


	long long int N, B, i, j;
	unsigned long long sum=0;

	long long int A[30001];
	
	scanf("%lld",&N);

	for (i = 1;i<=N;i++)scanf("%lld",&A[i]);

	for(i=1;i<=N-1;i++)
		for (j = i + 1;j <= N;j++) {
			B = A[i] ^ A[j];
			sum += B % aaa;
		}
	sum %= aaa;


	printf("%lld", sum);
	return 0;
}