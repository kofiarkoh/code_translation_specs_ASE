#include<stdio.h>
#include<stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

ll x,y;
int main(){
	scanf("%lld%lld",&x,&y);
	puts(llabs(x-y)<=1?"Brown":"Alice");
	return 0;
}