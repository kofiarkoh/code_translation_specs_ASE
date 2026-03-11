#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

#define _CRT_SECURE_NO_WARNINGS
#define TLong long long

int main(int argc, char const *argv[]){
	int n,k;
	int *a,*b,sum = 0,p;
	scanf("%d%d",&n,&k);
	a = (int *)malloc(sizeof(int) * n);
	b = (int *)malloc(sizeof(int) * n);
	for (int i = 0; i < n; ++i)	scanf(" %d%d",&a[i],&b[i]);
	for (p = 0; (p < n && sum < k); ++p)	sum += b[p];
	printf("%d\n", a[--p]);
	return 0;
}
