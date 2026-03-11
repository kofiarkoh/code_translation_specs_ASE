#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

#define P 1000000007

int comp(const void *a, const void *b){return *(int*)a-*(int*)b;}

int a[100000];
int main(void){
	int n, i;
	scanf("%d", &n);

	long sum=0, s=0;
	for(i=0; i<n; i++){
		scanf("%d", &a[i]);
		sum+=a[i];
		if(i&1) s+=a[i];
	}
	int b=sum-s*2;
	for(i=0; i<n; i++){
		printf("%d ", b);
		b=a[i]*2-b;
	}
	putchar(10);
}
