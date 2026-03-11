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
//#define LOCAL
#ifdef LOCAL
    #define DEBUG(...) printf(" "__VA_ARGS__)
#else
    #define DEBUG(...)
#endif /* LOCAL */
typedef struct{
	int t,num;
}City;
int comp(const void* a,const void* b){
	return ((City*)a)->t - ((City*)b)->t;
}
int d[51];
City city[101];
int used[51];
int n;
int check(int s){
	int now,cnt,id,diff,i,j;
	FOR(i,0,24){
		city[0].t=i,city[0].num=0;
		FOR(j,1,n+1){
			city[j*2-1].t=(i+d[j])%24; city[j*2-1].num=j;
			city[j*2].t=(i-d[j]+24)%24; city[j*2].num=j;
		}
		qsort(city,2*n+1,sizeof(City),comp);
		memset(used,0,sizeof(used));
		cnt=0; now=city[0].t+s;
		used[city[0].num]=1;
		FOR(j,1,2*n+1){
			if(city[j].t<now) continue;
			if(!used[city[j].num]){
				used[city[j].num]=1;
				cnt++; id=j;
				now=city[j].t+s;
			}
		}
		if(cnt==n){
			diff=city[id].t-city[0].t;
			if(MIN(diff,24-diff)>=s) return 1;
		}
	}
	return 0;
}
int main(void)
{
	int left,right,mid,i;
	scanf("%d",&n);
	FOR(i,1,n+1) scanf("%d",d+i);
	DEBUG("%d\n",check(4));
	left=0,right=13;
	while(right-left>1){
		mid=(left+right)/2;
		if(check(mid)) left=mid;
		else right=mid;
	}
	printf("%d\n",left);
	return 0;
}
    
    
