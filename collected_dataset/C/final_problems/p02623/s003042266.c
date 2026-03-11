#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
// 1,000,000,000,000,001
int main(void)
{
  int n, m, fl = 0;
  ll k, apos = 0, bpos = 0, alla = 0, timeb, max = 0, ch = 0;
  ll a[200001];
  ll b[200001];
  ll i, rest;
  scanf("%d%d%lld", &n, &m, &k);
  for (i = 0; i < n; i++)
  {
    scanf("%lld", &a[i]);
  }
  for (i = 0; i < n; i++)
  {
    alla += a[i];
    if (alla <= k)
    {
      apos++;
    }
    else
    {
      alla -= a[i];
      break;
    }
  }
  for (i = 0; i < m; i++)
  {
    scanf("%lld", &b[i]);
  }
  while (1)
  {
    if (bpos == m)
      break;
    alla += b[bpos];
    if (alla <= k)
    {
      bpos++;
    }
    else
    {
      alla -= b[bpos];
      break;
    }
  }
  //printf("%lld %lld\n", apos, bpos);
  if (apos == n && bpos == m)
  {
    printf("%d\n", n + m);
    return 0;
  }
  //このときはできるだけaを読んでから、隙間をbで埋めた状態
  //よってこの段階ではaからapos冊,bからbpos冊読んだ状態になっている(a[apos-1], b[bpos-1]までを使っている)
  rest = k - alla; //残った時間
  max = apos + bpos;
  i = 0;
  while (apos > 0)
  {
    rest += a[apos - 1]; //aの最後の１冊を読むのをやめた
    apos--;
    while (1) //この時間でBをどれだけ読めるか?
    {
      if (bpos == m)
      { //読み切った
        break;
      }
      rest -= b[bpos];
      if (rest >= 0)
      {
        bpos++;
      }
      else
      {
        rest += b[bpos];
        break;
      }
    }
    ch = bpos + apos;
    if (ch > max)
      max = ch;
  }
  printf("%lld\n", max);
  return 0;
}
