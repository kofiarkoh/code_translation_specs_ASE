#include <stdio.h>

#define MAX	100

int main(void)
{
	int n, k, x;
	int i, sum;
	
	scanf("%d", &n);
	scanf("%d", &k);
	
	if(k == 1)
		sum = 0;
	else{
		for(sum=0,i=0; i<n; i++){
			scanf("%d", &x);
			
			if(x < k-x)
				sum += 2*x;
			else
				sum += 2*(k-x);
		}
	}
	
	printf("%d", sum);	// Ans
	
	return 0;
}
