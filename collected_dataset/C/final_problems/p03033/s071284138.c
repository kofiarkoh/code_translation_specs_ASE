#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

//block linked list
#define Bsize 900 /*2sqrt(n)*/
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
ll Bans(block*b,int k){
	//リストのk番目を返す
	//リストにk個未満しかなければ適当に返す
	if((*b).cnt<=k){
		if((*b).nxt)return Bans((*b).nxt,k-(*b).cnt);
		else return -1;
	}
	return (*b).a[k];
}
void con(block*p,block*q){
	//qをpにマージしてqを消す
	rep(i,0,(*q).cnt)(*p).a[(*p).cnt+i]=(*q).a[i];
	(*p).cnt+=(*q).cnt;
	(*p).nxt=(*q).nxt;
	if((*q).nxt)(*(*q).nxt).pre=p;
	free(q);
}
int Bdel(block*b,int k){
	//リストからkを(存在すれば)1個削除する
	//存在すれば1、しなければ0を返す
	if((*b).cnt==0||(*b).a[(*b).cnt-1]<k){
		if((*b).nxt)return Bdel((*b).nxt,k);
		else return 0;
	}
	int i=0;
	for(;(*b).a[i]<k;i++);
	if((*b).a[i]!=k)return 0;
	for(;i<(*b).cnt-1;i++)(*b).a[i]=(*b).a[i+1];
	(*b).cnt--;
	
	if((*b).cnt==0&&(*b).pre)con((*b).pre,b);
	else if((*b).pre&&(*(*b).pre).cnt+(*b).cnt<Bsize/4)con((*b).pre,b);
	else if((*b).nxt&&(*(*b).nxt).cnt+(*b).cnt<Bsize/4)con(b,(*b).nxt);
	return 1;
}


ll a[400010],x[200010];
int c(const void*p,const void*q){
	if(a[*(ll*)p]<a[*(ll*)q])return -1;
	return 1;
}
block b;
ll idx[400010];
int main(){
	ll n,q;
	scanf("%lld%lld",&n,&q);
	rep(i,0,n){
		ll s,t;
		scanf("%lld%lld%lld",&s,&t,x+i);
		a[2*i  ]=s-x[i];
		a[2*i+1]=t-x[i];
	}
	rep(i,0,2*n)idx[i]=i;
	qsort(idx,2*n,sizeof(ll),c);
	
	ll crr=0;
	rep(_,0,q){
		ll d;
		scanf("%lld",&d);
		while(crr<2*n&&a[idx[crr]]<=d){
			if(idx[crr]%2)Bdel(&b,x[idx[crr]/2]);
			else Bpush(&b,x[idx[crr]/2]);
			crr++;
		}
		printf("%lld\n",Bans(&b,0));
	}
}
