#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

ll gcd(ll p,ll q){for(ll t;q;)t=p%q,p=q,q=t;return p;}


//sからtへの最大流 O(E flow)を横着してO(VV flow)
//*
#define VMAX 1010
int vcnt;
int foodoflag[VMAX];
int resflow[VMAX][VMAX];

int foododfs(int s,int t){
	if(s==t)return 1;
	foodoflag[s]=1;
	rep(i,0,vcnt)if(!foodoflag[i]&&resflow[s][i]>0){
		if(foododfs(i,t)){
			resflow[s][i]--;
			resflow[i][s]++;
			return 1;
		}
	}
	return 0;
}
int foodo(int s,int t){
	int ans=0;
	rep(i,0,vcnt)foodoflag[i]=0;
	while(foododfs(s,t)){
		ans++;
		rep(i,0,vcnt)foodoflag[i]=0;
	}
	return ans;
}
#undef VMAX
//*/

int a[510],b[510];
int main(){
	int n,m;
	while(scanf("%d%d",&n,&m),n){
		rep(i,0,n)scanf("%d",a+i);
		rep(j,0,m)scanf("%d",b+j);
		vcnt=n+m+2;
		rep(i,0,vcnt)rep(j,0,vcnt)resflow[i][j]=0;
		rep(i,0,n)resflow[0][i+1]=1;
		rep(j,0,m)resflow[j+n+1][m+n+1]=1;
		rep(i,0,n)rep(j,0,m)if(gcd(a[i],b[j])!=1)resflow[i+1][j+n+1]=1;
		printf("%d\n",foodo(0,n+m+1));
	}
}
