#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define max(p,q)((p)>(q)?(p):(q))

char s[2010][2010];
int ss[2010][2010];
int h,w;


ll sp,sh[2010],si[2010];
ll f(int base){
	//ヒストグラムの最大長方形
	ll ans=0;
	sp=0;
	rep(j,0,w){
		if(sp==0||sh[sp-1]<ss[base][j]){
			si[sp]=j;
			sh[sp]=ss[base][j];
			sp++;
		}else{
			int jj=-1;
			while(sp>0&&sh[sp-1]>=ss[base][j]){
				jj=si[sp-1];
				ll menseki=(sh[sp-1]+1)*(j-jj+1);//<--面積の計算が違う
				ans=max(ans,menseki);
				sp--;
			}
			if(jj!=-1){
				si[sp]=jj;
				sh[sp]=ss[base][j];
				sp++;
			}
		}
	}
	while(sp>0){
		ll jj=si[sp-1];
		ll menseki=(sh[sp-1]+1)*(w-jj+1);
		ans=max(ans,menseki);
		sp--;
	}
	return ans;
}

int main(){
	scanf("%d%d",&h,&w);
	rep(i,0,h)scanf("%s",s[i]);
	
	rep(i,0,h)rep(j,0,w)ss[i][j]=s[i][j]=='#'?0:1;
	//横のdiff
	rep(i,0,h)rep(j,0,w-1)ss[i][j]^=ss[i][j+1];
	//縦のdiff^1
	rep(i,0,h-1)rep(j,0,w-1)ss[i][j]^=ss[i+1][j]^1;
	
	h--;w--;
	//h*wの01配列s[i][j]であって、1からなる最大長方形を探す
	//ただし面積は(縦+1)*(横+1)で計算する
	
	//まず累積和っぽいものに変換
	//ss[i][j]は上方向にいくつ1が連続して連なっているか	
	rep(i,1,h)rep(j,0,w)if(ss[i][j]==1)ss[i][j]=ss[i-1][j]+1;
//	rep(i,0,h){rep(j,0,w)printf("%d ",ss[i][j]);puts("");}puts("");
	
	ll ans=max(h,w)+1;
	rep(i,0,h){
		ll t=f(i);
		ans=max(ans,t);
	}
	
	printf("%lld\n",ans);
	return 0;
}