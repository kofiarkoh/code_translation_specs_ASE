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
ll w[100000],d[100000];
int n,k;
int check(ll x){
	int i;
	ll cnt=0;
	FOR(i,0,n){
		if(x<w[i]) continue;
		cnt+=((x-w[i])/d[i]+1LL);
	}
	return cnt>=k;
}
int main(void)
{
	int i;
	ll left,right,mid;
	scanf("%d%d",&n,&k);
	FOR(i,0,n) scanf("%lld%lld",w+i,d+i);
	left=0,right=LLONG_MAX;
	while(right-left>1){
		mid=(right+left)/2;
		if(check(mid)) right=mid;
		else left=mid;
	}
	printf("%lld\n",right);
	return 0;
}