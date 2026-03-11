#include <stdio.h>
typedef long long int i64;
i64 calc(i64 x, int offset) {
  i64 s = 0;
  for(i64 i = offset; i < 10; i++)
    if(0 <= x + i && x + i < 10) s++;
  return s;
}
i64 solve(i64 n, i64 d) {
  i64 res = 0;
  switch(n) {
    case 1:
      return d == 0 ? 10LL : 0;
    case 2:
      return d % 9LL == 0 ? calc(d / 9LL, 1) : 0;
    case 3:
      return d % 99LL == 0 ? 10LL * calc(d / 99LL, 1) : 0;
    case 4:
      for(i64 i = -9; i <= 9; i++)
        if((d - 999LL * i) % 90LL == 0 && (d - 999LL * i) >= 0) res += calc(i, 1) * calc((d - 999LL * i) / 90LL, 0);
      return res;
    case 5:
      for(i64 i = -9; i <= 9; i++)
        if((d - 9999LL * i) % 990LL == 0 && (d - 9999LL * i) >= 0) res += 10LL * calc(i, 1) * calc((d - 9999LL * i) / 990LL, 0);
      return res;
    case 6:
      for(i64 i = -9; i <= 9; i++)
        for(i64 j = -9; j <= 9; j++)
          if((d - 99999LL * i - 9990LL * j) % 900LL == 0 && (d - 99999LL * i - 9990LL * j) >= 0) res += calc(i, 1) * calc(j, 0) * calc((d - 99999LL * i - 9990LL * j) / 900LL, 0);
      return res;
    case 7:
      for(i64 i = -9; i <= 9; i++)
        for(i64 j = -9; j <= 9; j++)
          if((d - 999999LL * i - 99990LL * j) % 9900LL == 0 && (d - 999999LL * i - 99990LL * j) >= 0) res += 10LL * calc(i, 1) * calc(j, 0) * calc((d - 999999LL * i - 99990LL * j) / 9900LL, 0);
      return res;
    case 8:
      for(i64 i = -9; i <= 9; i++)
        for(i64 j = -9; j <= 9; j++)
          for(i64 k = -9; k <= 9; k++)
            if((d - 9999999LL * i - 999990LL * j - 99900LL * k) % 9000LL == 0 && (d - 9999999LL * i - 999990LL * j - 99900LL * k) >= 0) res += calc(i, 1) * calc(j, 0) * calc(k, 0) * calc((d - 9999999LL * i - 999990LL * j - 99900LL * k) / 9000LL, 0);
      return res;
    case 9:
      for(i64 i = -9; i <= 9; i++)
        for(i64 j = -9; j <= 9; j++)
          for(i64 k = -9; k <= 9; k++)
            if((d - 99999999LL * i - 9999990LL * j - 999900LL * k) % 99000LL == 0 && (d - 99999999LL * i - 9999990LL * j - 999900LL * k) >= 0) res += 10LL * calc(i, 1) * calc(j, 0) * calc(k, 0) * calc((d - 99999999LL * i - 9999990LL * j - 999900LL * k) / 99000LL, 0);
      return res;
  }
}
int main(void) {
  i64 d, s = 0;
  scanf("%lld", &d);
  for(int i = 1; i <= 9; i++) s += solve(i, d);
  printf("%lld\n", s);
  return 0;
}
