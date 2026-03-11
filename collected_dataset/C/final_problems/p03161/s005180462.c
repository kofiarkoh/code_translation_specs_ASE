#include <stdio.h>
#include <stdlib.h>

#define MAXN 100000
#define INF 100000

int A[MAXN+1];
int DP[MAXN+1];


int flog (int n, int k)
{
	int i,tmp,j;
	int min;

	DP[1] = 0;

	for ( i = 2; i <= k; i++ ) {
		min = INF;
		for ( j = 1; j < i; j++ ) {
			tmp = DP[j] + abs(A[i]-A[j]);
			if ( min > tmp ) min = tmp;
		}
		DP[i] = min;
	}

	for ( i = k+1; i <= n; i++ ) {
		min = INF;
		for ( j = i-k; j < i; j++ ) {
			tmp = DP[j] + abs(A[j]-A[i]);
			if ( tmp < min ) min = tmp;
		}
		DP[i] = min;
	}

	return DP[n];

}

int main(void)
{
	int i,n,k;

	scanf("%d", &n); scanf("%d", &k);
	for ( i = 1; i <= n; i++ ) scanf("%d", &A[i]);

	printf("%d", flog(n,k));

	return 0;
}
