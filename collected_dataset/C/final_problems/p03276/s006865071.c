#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <math.h>
#include <limits.h>
#include <assert.h>
#define FOR(i,a,n) for(i=(a);i<(n);i++)
#define swap(type,a,b) do{type t=a;a=b;b=t;}while(0)
#define MAX(a,b) (((a)>(b))?(a):(b))
#define MIN(a,b) (((a)<(b))?(a):(b))
#define ll long long
#define INF 100000000
#define MOD 1000000007
#define SIZE 100005
#define LOCAL
#ifdef LOCAL
    #define DEBUG(...) printf(" "__VA_ARGS__)
#else
    #define DEBUG(...)
#endif /* LOCAL */
int comp(const void* a,const void* b){
	return *(int*)a-*(int*)b;
}
int x[100000];
int main(void)
{
	int n,k,i;
	ll l,r,res=1e18;
	scanf("%d%d",&n,&k);
	FOR(i,0,n) scanf("%d",&x[i]);
	FOR(i,0,n-k+1){
		l=x[i],r=x[i+k-1];
		if(l>=0) res=MIN(res,r);
		else if(r<=0) res=MIN(res,-l);
		else{
			res=MIN(res,2*(-l)+r);
			res=MIN(res,-l+2*r);
		}
	}
	printf("%lld\n",res);
	return 0;
}
    
    
    
    
    