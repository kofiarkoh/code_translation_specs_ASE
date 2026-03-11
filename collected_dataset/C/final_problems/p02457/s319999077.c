#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

//block linked list
#define Bsize 700 /*2sqrt(n)*/
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
	//リストにkが存在すれか答える
	//flagがtrueなら削除する
	if((*b).cnt==0||(*b).a[(*b).cnt-1]<k){
		if((*b).nxt)return Bfind((*b).nxt,k,flag);
		else return 0;
	}
	int i=0;
	for(;(*b).a[i]<k;i++);
	if((*b).a[i]!=k)return 0;
	if(flag)Bdel(b,i);
	return 1;
}
void Bdump(block*b,ll l,ll r){
	if((*b).cnt==0){
		if((*b).nxt)Bdump((*b).nxt,l,r);
	}else if((*b).a[0]>r)return;
	else if((*b).a[(*b).cnt-1]<l){
		if((*b).nxt)Bdump((*b).nxt,l,r);
	}
	else{
		rep(i,0,(*b).cnt)if(l<=(*b).a[i]&&(*b).a[i]<=r)printf("%lld\n",(*b).a[i]);
		if((*b).a[(*b).cnt-1]<=r&&(*b).nxt)Bdump((*b).nxt,l,r);
	}
}
#undef Bsize


block*b;
int main(){
	b=calloc(1,sizeof(block));
	int q;
	scanf("%d",&q);
	int cnt=0;
	while(q--){
		int t,x;
		scanf("%d%d",&t,&x);
		if(t==0){
			if(!Bfind(b,x,0)){
				Bpush(b,x);
				cnt++;
			}
			printf("%d\n",cnt);
		}else if(t==1){
			printf("%d\n",Bfind(b,x,0));
		}else if(t==2){
			if(Bfind(b,x,1))cnt--;
		}else{
			int y;
			scanf("%d",&y);
			Bdump(b,x,y);
		}
	}
}
