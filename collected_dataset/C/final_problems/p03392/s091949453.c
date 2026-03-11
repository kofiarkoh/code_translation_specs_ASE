/*
元の文字列が全て同じ文字からなれば1通り。
そうではなく元の文字列の長さが2なら2通り。
どちらにも該当しないとする。

「○○に操作を行って××にできるか？」は
・必要条件を考える（必要条件を与える不変量を考える）
・必要条件が実は十分条件になってないか考える
とすれば解けることが多い（要出典）

操作は"平均"を取ったと考えられそう。
これはa～cを0～2に変換してmod 3で考えることで正当化される。
つまり元の文字列から作れる文字列の必要条件は
・和のmod 3が元の文字列と等しい
・同じ文字が連続する箇所がある
となる。これで十分条件になるか？

前から貪欲でよさそう？
先頭がaのときbにできることを言えばよい
・2文字目がaのとき
初めてaではないものが出てくるところから前へいくとできる
例：aab→acc→bbc→baa→cca
aの数が増えたりb,cを入れ替えても同様に成り立つ
・2文字目がbのとき
aba→acc→bbc
abb→ccb→caa→bba
abc→できない(もしb..の形にできるなら必要条件からそれはbbb)
・2文字目がcのとき
ac→bb

abc→bbb（とそのpermutation）だけができない
4文字のときは
aabc→accc→bbcc→baac→babb→(bab→abbはできる)→abbb
babc→ccbc→caac→cabb→bbbb
cabc→bbbc→bbaa→bcca→bcbb→(bcb→cbbはできる)→cbbb
でできる

つまり4文字以上のとき、目指す文字列をつくるには「『同じものが連続している箇所』を含むような3文字の部分文字列」を残してそれ以外の部分を端から貪欲につくればよい。
まとめると
4文字以上のときは必要条件が十分条件になっている
3文字のときはabc→bbbとそのpermutationだけできない。他はできる

よって4文字以上のときは、上で述べた条件を満たすような文字列の個数を数えれば良い
dp[n][i][j][k]=n文字であって、現在の末尾の文字がi、和のmod 3がj、同じ文字が連続する箇所が(k?ある:ない)ようなものの数
としてdpでできる
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define MOD 998244353

char s[200010];
ll dp[200010][3][3][2];
//n文字であって、現在の末尾の文字がi、和のmod 3がj、同じ文字が連続する箇所が(k?ある:ない)ようなものの数

int main(){
	scanf("%s",s);
	
	//同じ文字が連続する箇所があるか、全てが同じ文字か、文字列の長さは
	int N=strlen(s);
	int con=0,ss[3]={};
	rep(i,0,N){
		ss[s[i]-'a']++;
		if(i&&s[i-1]==s[i])con=1;
	}
	
	if(ss[0]==N||ss[1]==N||ss[2]==N){
		//文字が1種類
		puts("1");
		return 0;
	}
	if(N==2){
		//2
		puts("2");
		return 0;
	}
	if(N==3){
		//3
		if(ss[0]==1&&ss[1]==1)puts("3");
		else printf("%d\n",6+!con);
		return 0;
	}
	
	//メイン
	dp[1][0][0][0]=dp[1][1][1][0]=dp[1][2][2][0]=1;
	rep(n,1,N)rep(i,0,3)rep(j,0,3)rep(k,0,2){
		rep(d,0,3)dp[n+1][d][(j+d)%3][k|(i==d)]=(dp[n+1][d][(j+d)%3][k|(i==d)]+dp[n][i][j][k])%MOD;
	}
	ll ans=0;
	rep(i,0,3)ans+=dp[N][i][(ss[1]+ss[2])%3][1];
	printf("%d\n",(ans+!con)%MOD);
}