#include <stdio.h>
#include <stdlib.h>

#define MAX (1000)

int updown(const void *x, const void *y){ return ((*((int *)x)) - (*((int *)y))); }
int main(void)
{
	int data[MAX];
	int minCost;
	int n, m;
	int i;
	
	while (1){
		scanf("%d %d", &n, &m);
		if (!(n * m)) break;
		for (i = 0; i < n; i++) scanf("%d", &data[i]);
		qsort(data, n, sizeof(int), updown);
		for (i = n % m; i < n; i += m) data[i] = 0;
		
		minCost = 0;
		for (i = 0; i < n; i++) minCost += data[i];
		
		printf("%d\n", minCost);
	}
	
	return (0);
}