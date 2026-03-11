#include <stdio.h>
#include <stdlib.h>

int compare(const void *a,const void *b)
{
	return *(int *)a - *(int *)b;
}

int main(void)
{
	int n,k,*h,i,j,min;
	scanf("%d %d",&n,&k);
	h = (int *)malloc(sizeof(int) * n);
	for (i=0;i<n;i++) scanf("%d",&h[i]);
	
	qsort(h,n,sizeof(int),compare);

	min = 1E9+1;
	for (i=0,j=k-1;j<n;i++,j++) {
		if (min > h[j]-h[i]) min = h[j]-h[i];
	}
	printf("%d\n",min);

	return 0;
}
