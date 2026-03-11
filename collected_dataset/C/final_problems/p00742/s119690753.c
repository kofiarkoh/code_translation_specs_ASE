/*  ACM-ICPC2009 国内予選 Problem C */
// http://www.waseda.jp/assoc-icpc2009/preliminary/contest/all_ja.html#section_C
// filename = pc2.c
// コンパイル：	cc -O2 pc2.c
// 実行方法：	./a.out < C0 > C0.result 等
// 確認方法：	diff C0.ans C0.result 等
// アルゴリズム：	覆面算に現れる文字に対して下位の桁に表れるものから順に
//		互いに異なる数値を割当る。
//		最下位桁に表れる文字への数値割当がすんだ時点で
//		覆面算の最下位桁の等式が成立するかをチェックし
//		不成立なら上位桁に対する割当をスキップする。
//		最下位桁の等式が成立するなら、次の文字に対する割当を継続し
//		下位２桁に表れる文字への数値割当がすんだ時点で
//		覆面算の下位2桁の等式が成立するかをチェックする。
//		このような処理を最上位桁まで繰り返す。

#include <stdio.h>
#include <string.h>

#define MAXN 12		// Nの最大値
#define MAXLEN 8	// 文字列の長さの最大値

int n;			// N,  3 <= n <= 12
char strs[MAXN][MAXLEN+1];	// STRING_i を strs[i-1][ ]に格納
int len[MAXN];		// 各 STRING の長さ
char c[10];		// STRING に表れる文字
int v[10];		// 各文字に割当てた値
int a[10][10];		// 各文字の係数. a[i][ ]は下からi桁目までの等式の係数
			// ただし、a[0][ ]は全体の等式
int ce[10];		// ce[i] = i文字目まで割当てた時点でチェックすべき等式番号
int md[10];		// 各文字が表れる最下位の桁位置
int maxlen = 0;
int nc;			// 使われている文字数
int zskip[10];		// 1 -> 値0は割当不可
// 桁位置の重み
int weight[] = {1,10,100,1000,10000,100000,1000000,10000000};
int count;		// 等式を満たす割当の個数
int nu[10];		// 未使用の数値リスト

// この時点でチェック可能な等式があればチェックする
// 数値を未割当の文字が残っている場合は、未使用の数値を順番に割当てる
void perm(int level)	// level = 数値割当済みの文字数
{
  int i,j;
  int val;
  int zl;
  int *ap,*vp;

  if(level == nc){	// 全ての文字に対して数値割当ずみ
    ap = &a[0][0];	// checkする条件式の係数の配列
    val = 0;
    for(i=0; i<nc; i++)
      val += v[i] * (*ap++);
    if(val == 0) count++;
    return;
  }
  if(ce[level] > 0){	// この時点でチェック可能な条件式がある
    ap = &a[ce[level]][0];	// その条件式の係数の配列
    val = 0;
    for(i=0; i<level; i++)
      val += v[i] * (*ap++);
    val = val % weight[md[level]-1];	// 下位 md[level] 桁で判定
    if(val != 0) return;	// 条件不成立なら残っている文字への数値割当不要
  }
  zl = zskip[level];		// zl = 1 なら 0 は割当不可
  for(i=0; i<10-level; i++){	// 未使用の数値を順に割当てていく
    val = nu[i];			// val は未使用の数値
    if(val==0 && zl) continue;		// 0が割当不可なら次へ
    v[level] = val;		// val を割当てる
    nu[i] = nu[9-level];	// val は使用済みとする
    perm(level+1);		// 再帰的に次の文字に数値を割当てていく
    nu[i] = val;		// val を未使用に戻す
  }
}

int main()
{
  int i,j,k;

  while(1){
    scanf("%d", &n);	// Nを入力
    if(n==0) break;	// N=0 なら終了
    for(i=0; i<n; i++){		// n 個の文字列を読み込む
      scanf("%s", &strs[i][0]);		// 文字列を読み込み
      len[i] = strlen(&strs[i][0]);	// その長さをセット
      if(len[i] > maxlen) maxlen = len[i];	// 文字列の最大長を更新
    }
    nc = 0;
    for(i=0; i<10; i++){
      c[i] = md[i] = -1;
      zskip[i] = 0;
    }
    for(i=0; i<10; i++)
      for(j=0;j<10; j++)
	a[i][j] = 0;
    for(i=1; i<=8; i++)
      for(j=0; j<n; j++){
	char *s;
	int d,w,lj;
	lj = len[j];
	s = &strs[j][0];
	if(i <= lj){
	  d = s[lj-i];
	  for(k=0; k<nc; k++){
	    if(c[k] == d) break;
	  }
	  if(k==nc){
	    md[k] = i;
	    c[nc++] = d;
	  }
	  if(i == lj && lj > 1) zskip[k] = 1;
	  w = weight[i-1];
	  if(j == n-1) w *= -1;
	  a[i][k] += w;	// 下位 i 桁の文字への数値割当決定時の条件式
	  a[0][k] += w;	// 全ての文字への数値割当決定時の条件式
	}
      }
    // 下位 i 桁以下の文字への数値割当決定時の条件式を求める
    for(i=2; i<= maxlen; i++)
      for(j=0; j<nc; j++)
	a[i][j] += a[i-1][j];
    // i 個の文字への数値割当決定時にチェックすべき条件式の番号を求める
    for(i=1; i<nc; i++)
      if(md[i-1] != md[i])
	ce[i] = md[i]-1;
      else
	ce[i] = 0;
    ce[0] = 0;
    count = 0;
    for(i=0; i<10; i++) nu[i] = i;
    perm(0);
    printf("%d\n",count);
  }
}

