#include<stdio.h>

int main() {
  long long n, a, b, c, d;
  int flag = 1;
  scanf("%lld %lld %lld %lld %lld", &n, &a, &b, &c, &d);
  long long sa_ab;
  if (a > b) {
    sa_ab = b-a;
  } else {
    sa_ab = a-b;
  }
  long long sa_cd = d-c;
  if (sa_ab > d*(n-1)) {
    flag = 0;
  }
  long long e = d*(n-1) - sa_ab;
  int count = 0;
  while (e >= 2*d && count < n) {
    e -= 2*d;
    count += 1;
  }
  while (e >= sa_cd && count < n) {
    e -= sa_cd;
    count += 1;
  }
  if (count >= n) {
    flag = 0;
  }
  if (flag == 1) {
    printf("YES\n");
  } else {
    printf("NO\n");
  }
  return 0;
}
