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
int comp(const void* a,const void* b){
	return *(int*)a-*(int*)b;
}
int d[50];
int cur[2],next[2];
int f1[2],f2[2];
int n;
int check(int s){
	int cnt,diff,f,i,j,k;
	f1[0]=f1[1]=f2[0]=f2[1]=0;
	cur[0]=cur[1]=0;
	FOR(i,0,n){
		next[0]=d[i],next[1]=(24-d[i])%24;
		FOR(j,0,2){
			cnt=f=0;
			FOR(k,0,2){
				if(f1[k]) continue;
				diff=abs(next[j]-cur[k]);
				DEBUG("%d\n",next[j]);
				if(diff>=s) cnt++; 
			}
			if(cnt>0) f=1;
			else f2[j]=1;
		}
		if(!f) return 0;
		f1[0]=f2[0],f1[1]=f2[1];
		cur[0]=next[0],cur[1]=next[1];
	}
	return 1;
}
int main(void)
{
	int left,right,mid,i;
	scanf("%d",&n);
	FOR(i,0,n) scanf("%d",d+i);
	DEBUG("%d\n",check(4));
	left=0,right=24;
	while(right-left>1){
		mid=(left+right)/2;
		if(check(mid)) left=mid;
		else right=mid;
	}
	printf("%d\n",left);
	return 0;
}
    
    
