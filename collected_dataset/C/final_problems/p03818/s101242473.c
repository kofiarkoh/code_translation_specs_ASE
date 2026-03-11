#include<stdio.h>
#include<stdlib.h>
#define ll long long
#define rep(i,l,r) for(ll i=(l);i<(r);i++)

ll c[100010];
ll ans,c2;
int main(){
	int n;
	scanf("%d",&n);
	rep(i,0,n){
		int t;
		scanf("%d",&t);
		c[t]++;
	}
	rep(i,0,100010){
		while(c[i]>2){
			ans++;
			c[i]-=2;
		}
		if(c[i]==2)c2++;
	}
	ans+=(c2+1)/2;
	printf("%d\n",n-2*ans);
	return 0;
}