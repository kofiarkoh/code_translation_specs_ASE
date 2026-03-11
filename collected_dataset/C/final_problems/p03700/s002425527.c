#include<stdio.h>
#include<math.h>

int main() {
  long long n, a, b;
  long long h[100001] = {};
  int i;
  scanf("%lld %lld %lld", &n, &a, &b);
  long long big = 0;
  for (i = 0; i < n; i++) {
    scanf("%lld", &h[i]);
    if (big < h[i]) {
      big = h[i];
    }
  }
  long long imin = 0;
  long long imax = big / b + 1;
  long long imid;
  while (imax-1 > imin) {
    long long count = 0;
    long long temp[100001] = {};
    imid = imin + (imax - imin) / 2;
    for (i = 0; i < n; i++) {
      temp[i] = h[i];
      temp[i] -= b * imid;
      if (temp[i] < 0) {
	temp[i] = 0;
      }
    }
    for (i = 0; i < n; i++) {
      count += temp[i] / (a-b);
      if (temp[i] % (a-b) != 0) {
	count += 1;
      }
    }
    if (count > imid) {
      imin = imid;
    } else {
      imax = imid;
    }
    /*printf("%lld %lld\n", imin, imax);*/
  }
  printf("%lld\n", imax);
  return 0;  
}
