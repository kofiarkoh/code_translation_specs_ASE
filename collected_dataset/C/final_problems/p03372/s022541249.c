#include <stdio.h>

typedef long long ll;

ll x[100005];
ll v[100005];
ll f1[100005];
ll f2[100005];
ll g[100005];
ll g2[100005];
ll tmp[100005];

ll max(ll a,ll b){
	if(a>=b){return a;}
	else{return b;}
}
ll min(ll a,ll b){
	if(a<=b){return a;}
	else{return b;}
}

int main(){

	int N;
	ll C;

	scanf("%d %lld",&N,&C);

	int i;

	for(i=1;i<=N;i++){
		scanf("%lld %lld",&x[i],&v[i]);
	}

	tmp[0]=0ll;
	for(i=1;i<=N;i++){
		tmp[i]=v[i]+tmp[i-1];
	}
	f1[0]=0ll;
	for(i=1;i<=N;i++){
		f1[i]=tmp[i]-x[i];
	}
	tmp[N]=v[N];
	for(i=N-1;i>=1;i--){
		tmp[i]=tmp[i+1]+v[i];
	}
	for(i=1;i<=N;i++){
		f2[i]=tmp[i]-(C-x[i]);
	}

	g[0]=f1[0];
	ll max_f=g[0];
	for(i=1;i<=N;i++){
		if(max_f<f1[i]){
			max_f=f1[i];
			g[i]=max_f;
		}
		else{
			g[i]=g[i-1];
		}
	}

	g2[0]=0ll;
	ll max_f2=0ll;
	for(i=1;i<=N;i++){
		if(max_f2<f1[i]-x[i]){
			max_f2=f1[i]-x[i];
			g2[i]=max_f2;
		}
		else{
			g2[i]=g2[i-1];
		}
	}

	ll ans=0ll;

	for(i=1;i<=N;i++){
		ans=max(ans,f1[i]);
	}
	for(i=N;i>=1;i--){
		ans=max(ans,f2[i]);
	}

	for(i=1;i<=N;i++){
		ll OA=g[i-1];
		ll OB=f2[i];
		ans=max(OB-(C-x[i])+OA,ans);
	}
	for(i=1;i<=N;i++){
		ll OA2=g2[i-1];
		ll OB=f2[i];
		ans=max(OA2+OB,ans);
	}

	printf("%lld\n",ans);

	return 0;
}