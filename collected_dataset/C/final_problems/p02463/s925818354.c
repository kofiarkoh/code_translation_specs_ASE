#include <stdio.h>
#include <stdlib.h>


#define	ll_int	long long int
#define	INF	1000000001


int main()
{
	int n, m;
	ll_int *A, *B;
	int count;
	int i, j, k;

	scanf("%d", &n);
	A = (ll_int *)malloc(sizeof(ll_int) * (n + 1));
	for(i = 0; i < n; i++)
		scanf("%lld", &A[i]);
	A[n] = INF;

	scanf("%d", &m);
	B = (ll_int *)malloc(sizeof(ll_int) * (m + 1));
	for(i = 0; i < m; i++)
		scanf("%lld", &B[i]);
	B[m] = INF;


	// 表示
	j = k = count = 0;
	for(i = 0; i < n + m - count; i++){
		if(A[j] < B[k])
			printf("%lld\n", A[j++]);
		else if(B[k] < A[j])
			printf("%lld\n", B[k++]);
		else{
			printf("%lld\n", A[j++]);
			k++;
			count++;
		}
	}

	return 0;
}
