#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAXVAL 2147483647

#define min(a, b) ((a)<(b)?(a):(b))

int *a, *b, v, n;
unsigned int s, t;

int find_min(unsigned int k, unsigned int l, unsigned int r)
{
  if (r<=s || t<=l) return MAXVAL;
  if (s<=l && r<=t) return a[k] + b[k];
  int m1 = find_min((k<<1)+1, l, (l+r)>>1);
  int m2 = find_min((k<<1)+2, (l+r)>>1, r);
  return min(m1, m2) + a[k]; 
}

void update(unsigned int k, unsigned int l, unsigned int r)
{
  if (r<=s || t<=l) return;
  if (s<=l && r<=t) {
    a[k] += v;
  } else {
    unsigned int k1 = (k<<1)+1, k2 = k1 + 1;
    update(k1, l, (l+r)>>1);
    update(k2, (l+r)>>1, r);
    b[k] = min(a[k1]+b[k1], a[k2]+b[k2]);
  }
}

int main()
{
  int i, w, sign;
  char *p, buf[64] = {0};
  buf[62] = '\n';
  fgets(buf, 64, stdin);
  for (n=0,p=buf; *p>' '; n=n*10+*(p++)-'0');
  w = n>1 ? pow(2, (int)ceil(log2(n)))-1 : 0;
  b = (a = (int*)malloc(sizeof(int)*(w+n)*2)) + w + n;
  w++;
  while (fgets(buf, 64, stdin)) {
    for (s=0,p=buf+2; *p>' '; s=s*10+*(p++)-'0');
    for (t=0,p++; *p>' '; t=t*10+*(p++)-'0');
    t++;
    if (*buf=='0') { // update(s, t, x)
      if (*(++p)=='0') continue;
      sign=0;
      if (*p=='-') {sign=1;p++;}
      for (v=0; *p>' '; v=v*10+*(p++)-'0');
      if (sign) v=-v;
      update(0, 0, w);
    } else { // find(s, t)
      v = find_min(0, 0, w);
      if (v<0) {
	putchar('-');
	v = -v;
      }
      if (v<10) {
	putc_unlocked('0'+v, stdout);
	putc_unlocked('\n', stdout);
      } else {
	for (i=62; v>0; buf[--i]='0'+v%10,v/=10);
	fputs(buf+i, stdout);
      }
    }
  }
  return 0;
}

