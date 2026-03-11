#include <stdio.h>
#include <stdlib.h>


#define	ll_int	long long int


int main()
{
	int n, m;
	ll_int *A, *B;
	int count;
	int i, j, k;

	scanf("%d", &n);
	A = (ll_int *)malloc(sizeof(ll_int) * n);
	for(i = 0; i < n; i++)
		scanf("%lld", &A[i]);

	scanf("%d", &m);
	B = (ll_int *)malloc(sizeof(ll_int) * m);
	for(i = 0; i < m; i++)
		scanf("%lld", &B[i]);


	// A, Bの要素を比較していく
	j = k = count = 0;
	for(i = 0; i < n + m - count; i++){
		if(A[j] == B[k]){
			printf("%lld\n", A[j]);
			j++;
			k++;
			count++;
		}else if(A[j] < B[k])
			j++;
		else if(B[k] < A[j])
			k++;
	}

	return 0;
}
