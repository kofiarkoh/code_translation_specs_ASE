#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

char s[99][99];
double x[99];
int idx[99];

int c(const void*a,const void*b){
	if(x[*(int*)a]>x[*(int*)b])return -1;
	if(x[*(int*)a]<x[*(int*)b])return 1;
	return strcmp(s[*(int*)a],s[*(int*)b]);
}

int main(){
	ll n;
	while(scanf("%lld",&n),n){
		rep(i,0,n){
			ll a[9];
			scanf(" %s",s[i]);
			rep(k,0,9)scanf("%lld",a+k);
			x[i]=(double)(a[8]*a[7]*a[6]-a[0])/(a[1]+a[2]+a[3]+(a[4]+a[5])*a[8]);
			idx[i]=i;
		}
		qsort(idx,n,sizeof(int),c);
		rep(i,0,n)printf("%s\n",s[idx[i]]);
		puts("#");
	}
}
