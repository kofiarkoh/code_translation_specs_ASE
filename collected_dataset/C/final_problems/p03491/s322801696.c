#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

char s[200010];
ll ptr[100010];

char t[200010];
ll tlen;
ll ans;
ll err;

void nxt(){
	//tを、tlen文字以下であって辞書順でtの直後にあたる文字列に置き換える
	//そのようなものが存在しなければ(⇔tが1のみからなる文字列なら)errフラグを立てる
	while(tlen>0&&t[tlen-1]=='1')tlen--;
	if(tlen==0)err=1;
	t[tlen-1]='1';
	t[tlen]=0;
}

int cmp(const void*a,const void*b){
	return strcmp(s+*(ll*)a,s+*(ll*)b);
}
int main(){
	ll n,k;
	scanf("%lld%lld",&n,&k);
	rep(i,0,n){
		scanf("%s",s+ptr[i]);
		ptr[i+1]=ptr[i]+strlen(s+ptr[i])+1;
	}
	qsort(ptr,n,sizeof(ll),cmp);
	s[ptr[n]]='2';//番兵
	
	
	ll i=0;
	while(!err){
		//strcmpの計算量大丈夫？
		if(!strcmp(t,s+ptr[i])){
			//完全に一致するなら次へ
			i++;
			nxt();
		}else if(!strncmp(t,s+ptr[i],tlen)){
			//prefixなら下へ潜る
			t[tlen++]='0';
			t[tlen]=0;
		}else{
			//prefixでないなら、ここは使える部分木の根
			//grundy数を更新して次へ
//			puts(t);fflush(0);
			ll temp=k-tlen+1;
			ans^=(temp&-temp);
			nxt();
		}
	}
	//printf("%lld ",ans);
	puts(ans?"Alice":"Bob");
	return 0;
}