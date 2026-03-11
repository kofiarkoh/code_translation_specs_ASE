#include <stdio.h>
#include <math.h>

int main(void) {
	int n, a[1111], i, j, min, max, cmin=0, c[1111]={};
	
	scanf("%d", &n);
	
	for(i=0;i<n;i++){
		scanf("%d", &a[i]);
	}
	
	min=a[0];
	max=a[0];
	
	for(i=1;i<n;i++){
		if(min>a[i])
		min=a[i];
		if(max<a[i])
		max=a[i];
	}
	
	for(i=0;i<n;i++){
		cmin+=pow(a[i]-min, 2);
	}
	
	for(i=min;i<=max;i++){
		for(j=0;j<n;j++){
			c[i]+=pow(a[j]-i, 2);
		}
		if(c[i]<cmin)
		cmin=c[i];
	}
	
	printf("%d\n", cmin);
	
	
	return 0;
}
