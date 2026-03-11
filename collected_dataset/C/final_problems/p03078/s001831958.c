#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int comp(const void *a,const void *b){
	long long int da = *(long long int*)a;
    long long int db = *(long long int*)b;
    if(da > db)
    	return -1;
    else if(da == db)
    	return 0;
    else return 1;
}

int main()
{
	int a,b,c;
	long long x[1001],y[1001],z[1001];
	long long k1,k,sum = 0;
	long long s[300000];
	memset(x,0,1001);
	memset(y,0,1001);
	memset(z,0,1001);
	
	scanf("%d %d %d %lld",&a,&b,&c,&k);
	for(int i = 0;i < a;i++)
		scanf("%lld",&x[i]);
	for(int i = 0;i < b;i++)
		scanf("%lld",&y[i]);
	for(int i = 0;i < c;i++)
		scanf("%lld",&z[i]);
	
	qsort(x,a,sizeof(x[0]),comp);
	qsort(y,b,sizeof(y[0]),comp);
	qsort(z,c,sizeof(z[0]),comp);
	
	for(int i = 0;i < a;i++){
		for(int j = 0;j < b;j++){
			for(int g = 0;g < c;g++){
				if((i+1)*(j+1)*(g+1) <= k){
					s[sum] = x[i] + y[j] + z[g];
					sum++;
				}
			}
		}
	}
	
	qsort(s,sum,sizeof(s[0]),comp);
	for(int i = 0;i < k;i++)
		printf("%lld\n",s[i]);
	return 0;
}