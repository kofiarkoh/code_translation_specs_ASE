#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define LIMIT 100010
//union-find
int uniroot[LIMIT+10],unicnt[LIMIT+10];
void ufinit(int n){for(int i=0;i<n;i++){uniroot[i]=i;}}//unicnt[i]=1;}}
int root(int x){return uniroot[x]!=x?uniroot[x]=root(uniroot[x]):x;}
int same(int x,int y){return root(x)==root(y);}
void uni(int x,int y){if((x=root(x))==(y=root(y)))return;uniroot[x]=y;}//unicnt[y]+=unicnt[x];}

int a[100010];
int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	ufinit(n+1);
	rep(i,0,n){
		int t;
		scanf("%d",&t);
		a[t]=i+1;
	}
	rep(i,0,m){
		int p,q;
		scanf("%d%d",&p,&q);
		uni(p,q);
	}
	ll ans=0;
	rep(i,1,n+1)if(same(a[i],i))ans++;
	printf("%lld\n",ans);
	return 0;
}

