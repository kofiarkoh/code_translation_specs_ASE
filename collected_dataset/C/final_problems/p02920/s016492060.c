#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

//block linked list
#define Bsize 300 /*2sqrt(n)*/
typedef struct block{
	int cnt;
	ll a[Bsize+1];
	struct block*pre;
	struct block*nxt;
}block;
void Bpush(block*b,ll x){
	//リストにxを追加する
	if((*b).nxt&&(*(*b).nxt).a[0]<x){
		//次へ
		Bpush((*b).nxt,x);
		return;
	}
	int i=(*b).cnt;
	for(;i>0&&(*b).a[i-1]>x;i--)(*b).a[i]=(*b).a[i-1];
	(*b).a[i]=x;
	(*b).cnt++;
	if((*b).cnt>=Bsize){
		//分割
		block*nn=(block*)malloc(sizeof(block));
		(*nn).cnt=Bsize-Bsize/2;
		rep(i,0,Bsize-Bsize/2)(*nn).a[i]=(*b).a[i+Bsize/2];
		(*nn).pre=b;
		(*nn).nxt=(*b).nxt;
		if((*b).nxt)(*(*b).nxt).pre=nn;
		(*b).cnt=Bsize/2;
		(*b).nxt=nn;
	}
}
void con(block*p,block*q){
	//qをpにマージしてqを消す
	rep(i,0,(*q).cnt)(*p).a[(*p).cnt+i]=(*q).a[i];
	(*p).cnt+=(*q).cnt;
	(*p).nxt=(*q).nxt;
	if((*q).nxt)(*(*q).nxt).pre=p;
	free(q);
}
void Bdel(block*b,int idx){
	//idx番目を削除する(必ずidx個以上ある)
	for(;idx<(*b).cnt-1;idx++)(*b).a[idx]=(*b).a[idx+1];
	(*b).cnt--;
	
	if((*b).cnt==0&&(*b).pre)con((*b).pre,b);
	else if((*b).pre&&(*(*b).pre).cnt+(*b).cnt<Bsize/4)con((*b).pre,b);
	else if((*b).nxt&&(*(*b).nxt).cnt+(*b).cnt<Bsize/4)con(b,(*b).nxt);
}
ll Bans(block*b,int k,int flag){
	//リストのk番目を返す
	//リストにk個未満しかなければ適当に返す
	//flagがtrueなら削除する
	if((*b).cnt<=k){
		if((*b).nxt)return Bans((*b).nxt,k-(*b).cnt,flag);
		else return -1;//<-例外
	}
	ll ans=(*b).a[k];
	if(flag)Bdel(b,k);
	return ans;
}
int Bfind(block*b,ll k,int flag){
	//リストにkより大きいものが存在すればそれを返す
	//flagがtrueなら削除する
	if((*b).cnt==0||(*b).a[(*b).cnt-1]<=k){
		if((*b).nxt)return Bfind((*b).nxt,k,flag);
		else return 1;
	}
	int i=0;
	for(;(*b).a[i]<=k;i++);
	ll ans=(*b).a[i];
	if(flag)Bdel(b,i);
	return ans;
}
#undef Bsize

block*B1,*B2,*B3,*temp;
int main(){
	int n;
	scanf("%d",&n);
	B1=(block*)malloc(sizeof(block));
	B2=(block*)malloc(sizeof(block));
	B3=(block*)malloc(sizeof(block));
	rep(i,0,1<<n){
		int t;
		scanf("%d",&t);
		Bpush(B3,-t);
	}
	ll t=Bans(B3,0,1);
	Bpush(B1,t);
	rep(i,0,n){
		rep(j,0,1<<i){
			ll t=Bans(B1,0,1);
			ll tt=Bfind(B3,t,1);
			if(tt==1){
				puts("No");
				return 0;
			}
			Bpush(B2,tt);
			Bpush(B2,t);
		}
		temp=B1;B1=B2;B2=temp;
	}
	puts("Yes");
}