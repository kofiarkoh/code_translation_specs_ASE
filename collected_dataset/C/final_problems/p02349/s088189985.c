#include <stdio.h>
#include <stdlib.h>
#include <math.h>

const int pow2[] = {0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535, 131071 };

int *a, y;

int sum(int k, int l, int r)
{
    if (y<l)  return 0;
    if (r<=y) return a[k];
    int t = (l+r)>>1;
    return sum(k<<1, l, t) + sum((k<<1)+1, t+1, r);
}

int main()
{
  int i, j, k, n, z, s, t, u, x;
  char buf[32], *p;
  buf[31] = 0;
  buf[30] = '\n';
  fgets(buf, 32, stdin);
  for (n=0,p=buf; *p!=' '; n=n*10+*(p++)-'0');
  n++;
  u = pow2[(int)ceil(log2(n))];
  a = (int*)malloc((u+n+1)*sizeof(int));
  
  while (fgets(buf, 32, stdin)) {
    p = buf + 2;
    if (buf[0]=='0') {
      for (s=0; *p!=' ';  s=s*10+*(p++)-'0');
      for (t=0, p++; *p!=' ';  t=t*10+*(p++)-'0');
      for (x=0, p++; *p!='\n'; x=x*10+*(p++)-'0');
      for (k=u+s,a[k]+=x; k>1; a[k]=a[j]+a[j+1]) {
        j=k&~1;  k>>=1;
      }
      for (k=u+t+1,a[k]-=x; k>1; a[k]=a[j]+a[j+1]) {
        j=k&~1;  k>>=1;
      }
    } else {
      for (y=0; *p!='\n'; y=y*10+*(p++)-'0');
      z = sum(1, 1, u+1);
      if (z==0) {
        putc_unlocked('0', stdout);
        putc_unlocked('\n', stdout);
      } else {
        for (k=1; k*10<=z; k*=10);
        for (;k>0; k/=10) {
          putc_unlocked('0'+z/k, stdout);
          z %= k;
        }
        putc_unlocked('\n', stdout);
      }
    }
  }
  return 0;
}

