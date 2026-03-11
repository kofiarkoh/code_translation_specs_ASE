#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

int cnt;
ll num[60000],memo[60000];
ll a[3],d;

int uniquell(ll*a,ll*ans,int cnt){
	sortup(a,cnt);
	int c=0;
	ans[0]=a[0];
	rep(i,1,cnt)if(a[i]!=ans[c])ans[++c]=a[i];
	return c+1;
}

int bs(ll x){
	int l=0,r=cnt;
	while(r-l>1){
		int m=(l+r)/2;
		if(num[m]<=x)l=m;
		else r=m;
	}
	return l;
}

#define chmin(x,y)(x>y?x=y:0)
ll f(ll x){
	if(x==0)return 0;
	if(x==1)return d;

	int idx=bs(x);
	if(memo[idx])return memo[idx]-1;
	
	ll ans=1e12;
	if(x<1e12/d)ans=x*d;
	
	ll t;
	int div[]={2,3,5};
	rep(i,0,3){
		int m=div[i];
		if(x%m<=(m-1)/2){
			t=f(x/m);
			chmin(ans,t+a[i]+d*(x%m));
		}else{
			t=f(x/m+1);
			chmin(ans,t+a[i]+d*(m-x%m));
		}
	}

	memo[idx]=ans+1;
	return ans;
}

int main(){
	int t;
	scanf("%d",&t);
	while(t--){
		ll n;
		scanf("%lld%lld%lld%lld%lld",&n,a,a+1,a+2,&d);
		cnt=0;
		for(ll temp0=n;temp0;temp0/=2){
			for(ll temp1=temp0;temp1;temp1/=3){
				for(ll temp2=temp1;temp2;temp2/=5){
					num[cnt++]=temp2;
					num[cnt++]=temp2+1;
				}
			}
		}
		cnt=uniquell(num,num,cnt);
		rep(i,0,cnt)memo[i]=0;
		printf("%lld\n",f(n));
	}
}