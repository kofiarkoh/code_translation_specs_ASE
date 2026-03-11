#include <stdio.h>

int suming(long long a[100000], int i, int j, int k)
{
	if(k < i)
		return suming(a, i, j, k+1) + a[j+k];
	else
		return 0;
}

int main(void)
{
	int n;
	long long a[100000];
	int count = 0;
	int i, j;
	
	/* ----- Input ----- */
	scanf("%d", &n);
	for(i=0; i<n; i++)
		scanf("%ld", &a[i]);
	
	
	/* ----- To Do ----- */
	for(i=2; i<=n; i++){
		for(j=0; j<=n-i; j++){
			long long sum = 0;
			
			sum = suming(a, i, j, 0);
			
			if(sum == 0)
				count++;
		}
	}
	
	/* ----- Output ----- */
	printf("%d\n", count);
	
	
	return 0;
}
