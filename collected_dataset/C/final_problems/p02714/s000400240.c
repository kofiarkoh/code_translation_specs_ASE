#include <stdio.h>  /*入出力*/
#include <stdlib.h> /*標準ライブラリ*/
#include <string.h> /*文字列*/
#include <math.h>   /*計算*/
#include <assert.h> /*確認*/
#include <ctype.h>  /*文字操作*/
#include <stdbool.h>/*bool型*/
#include <limits.h>/*限界*/

//---------------------------------------------------

#define rep(i, n) for(int  i = 0; i < (n); i++)
#define max(a,b) ((a>b)?a:b)
#define ABS(a) (((a)>(0))?a:-(a))
#define LL long long
#define MOD 1000000007//10^9+7(素数)

//---------------------------------------------------
int main(void)
{
  int N;
  char s[5000];
  LL sum = 0;
  scanf("%d %s",&N, &s );
  for (int i = 0; i < N; i++) {
    for (int j = i+1; j < N; j++) {
      for (int k = j+1; k < N; k++) {
        if (s[i] != s[j] && s[i] != s[k] && s[j] != s[k]) {
          if (j-i != k-j) {
            sum ++;
          }
        }
      }
    }
  }
  printf("%lld\n", sum);
}
