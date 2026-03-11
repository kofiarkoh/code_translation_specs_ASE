#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <assert.h>
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
ll gcd(ll p,ll q){while(q){ll t=p%q;p=q;q=t;}return p;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

//KMP法
//検索したい文字列をmakekmpに食わせてから
//kmpsearchを呼ぶ
//*
int*makekmp(char*t,int tlen){
	int*tt=calloc(tlen+1,sizeof(int));
	int len=0;
	tt[0]=-1;
	for(int pos=1;pos<tlen;pos++,len++){
		if(t[pos]==t[len]){
			//同じなのに失敗したら、失敗
			tt[pos]=tt[len];
		}
		else{
			//違うとき失敗したら、そこからチェック
			tt[pos]=len;
			//一致するところまで戻る
			while(len>=0&&t[pos]!=t[len])len=tt[len];
		}
	}
	tt[tlen]=len;
	return tt;
}
int kmpsearch(char*s,char*t,int*tt){
	int k=0;
	for(int j=0;s[j];){
		if(s[j]==t[k]){
			j++,k++;
			if(!t[k]){
				//最後まで行ったら、最後で失敗扱い
				return 1;
			}
		}else{
			//失敗
			k=tt[k];
			if(k==-1)j++,k++;//1文字目
		}
	}
	return 0;
}
//*/

char s[2000010];
char t[2000010];
int main(){
	scanf("%s%s",s,t);
	int sn=strlen(s);
	int tn=strlen(t);
	int sss=sn;
	while(sss<1.5e6)memcpy(s+sss,s,sn),sss+=sn;

	int length=(sss-(tn-1))/tn*tn;
	int ttt=tn;
	while(ttt+tn<=length)memcpy(t+ttt,t,tn),ttt+=tn;
	int*p=makekmp(t,ttt);
	if(kmpsearch(s,t,p)){
		puts("-1");
		return 0;
	}
	int l=0,r=ttt/tn;
	while(r-l>1){
		int m=(l+r)/2;
		t[tn*m]=0;
		if(kmpsearch(s,t,p))l=m;
		else r=m;
		t[tn*m]=t[0];
	}
	printf("%d",l);
}