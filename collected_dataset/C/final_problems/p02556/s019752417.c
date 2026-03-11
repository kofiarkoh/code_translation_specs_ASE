#include <stdio.h>

int main(int argc, char *argv[])
{
	long long n;
	scanf("%lld",&n);
	long long a[n],b[n],max=-1,min=100000000000000,i,c[n];
	for(i=0;i<n;i++){
		scanf("%lld %lld",&a[i],&b[i]);
		c[i]=a[i]+b[i];
	}
	for(i=0;i<n;i++){
		if(c[i]>max)
			max=c[i];
		if(c[i]<min)
			min=c[i];
	}
	printf("%lld\n",max-min);
}