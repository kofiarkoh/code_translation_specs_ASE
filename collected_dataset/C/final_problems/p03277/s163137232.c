#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

//座標圧縮
int cocomp(ll*a,int n){
	//0～cnt-1に値を圧縮
	ll*b=(ll*)malloc(sizeof(ll)*n);
	memcpy(b,a,sizeof(ll)*n);
	sortup(b,n);
	int cnt=1;
	rep(r,1,n)if(b[r]!=b[cnt-1])b[cnt++]=b[r];
	rep(i,0,n){
		int l=0,r=cnt;
		while(r-l>1){int m=(l+r)/2;if(b[m]>a[i])r=m;else l=m;}
		a[i]=l;
	}
	free(b);
	return cnt;
}

//seg木
int segNUM;
ll segN[1<<21],*seg;
void seguse(int n){segNUM=n;seg=segN+segNUM;}

//seg[]に値を与えてから初期化
void seginit(){
	for(int node=segNUM-1;node;node--)segN[node]=segN[node*2]+segN[node*2+1];//sum
}
void segupdate(int node,ll x){
	//seg[node]をxに更新
	node+=segNUM;
	segN[node]=x;
	while(node/=2){
		segN[node]=segN[node*2]+segN[node*2+1];//sum
	}
}

ll segcalcsub(int a,int b,int k,int l,int r){
	//segN[k]に[l,r)が対応する
	//disjointなとき
	if(r<=a||b<=l)return 0;//sum
	//完全に含むとき
	if(a<=l&&r<=b)return segN[k];
	//どちらもでもないとき
	int left=segcalcsub(a,b,k*2,l,(l+r)/2),right=segcalcsub(a,b,k*2+1,(l+r)/2,r);
	return left+right;//sum
}

//[a,b)に対して答える
ll segcalc(int a,int b){return segcalcsub(a,b,1,0,segNUM);}



ll n,nn;
ll a[100010],b[100010];
ll s[100010];
ll check(ll k){
	//aの連続部分列であって、k以上の要素をceil((長さ)/2)個含むものはいくつあるか
	//⇔k未満なら-1,k以上なら+1としたとき、総和が0以上になる連続部分列はいくつあるか
	//⇔累積和をとってseg木でえい
	s[0]=0;
	rep(i,0,n){
		s[i+1]=s[i];
		if(a[i]>=k)s[i+1]++;
		else s[i+1]--;
	}
	ll cnt=cocomp(s,n+1);
//	rep(i,0,n+1)printf("%lld ",s[i]);puts("");
	ll ans=0;
	rep(i,0,cnt+5)seg[i]=0;
	seginit();
	rep(i,0,n+1){
		ans+=segcalc(0,s[i]+1);
		segupdate(s[i],seg[s[i]]+1);
	}
	return ans;
}


int main(){
	scanf("%lld",&n);
	nn=n*(n+1)/2;
	rep(i,0,n){
		scanf("%lld",a+i);
		b[i]=a[i];
	}
	sortup(b,n);

	seguse(1<<17);
	//にぶたん
	ll l=0,r=n;//lはok,rはng
	while(r-l>1){
		ll m=(l+r)/2;
//		printf("%lld %lld\n",b[m],check(b[m]));
		if(check(b[m])>=(nn+1)/2)l=m;
		else r=m;
	}
	
	printf("%lld\n",b[l]);
	return 0;
}