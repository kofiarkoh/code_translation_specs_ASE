#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int step,a[200],n,i,j;

void swap (int* x,int* y){
	int z;
	z=*x;
	*x=*y;
	*y=z;
}

int main(){
	while(scanf("%d",&n)*n){
		for(i=0;i<n;i++)scanf("%d",&a[i]);
		step=0;

		for(i=0;i<n-1;i++){
			for(j=0;j<n-i-1;j++){
				if(a[j]>a[j+1]){
					swap(&a[j],&a[j+1]);
					step++;
				}
			}
		}
		printf("%d\n",step);
	}
	return 0;
}