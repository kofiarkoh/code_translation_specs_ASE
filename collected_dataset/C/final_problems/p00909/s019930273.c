#include <stdio.h>
#include <stdlib.h>

#define N 100000

int main()
{
  int i, n, q, v, x, y, z, dx, dy, x0, y0, ex, rx, sx, tmp;
  char *p, buf[32];
  buf[31]=0;
  buf[30]='\n';
  int *s = (int*)malloc(sizeof(int)*N);
  int *d = (int*)malloc(sizeof(int)*N);
  while (fgets(buf, 32, stdin)) {
    for (n=0,p=buf; *p!=' '; n=n*10+*(p++)-'0'); 
    for (q=0,p++; *p!='\n'; q=q*10+*(p++)-'0'); 
    if (n==0 && q==0) break;
    for (i=1; i<=n; s[i]=i,d[i++]=0);
    for (int t=0; t<q; t++) {
      fgets(buf, 32, stdin);
      for (x=0,p=buf+2; *p!=' '; x=x*10+*(p++)-'0');
      for (y=0,p++; *p>='0'; y=y*10+*(p++)-'0');

      if (buf[0]=='!') { // relate x y z
	for (z=0,p++; *p!='\n'; z=z*10+*(p++)-'0');
	for (x0=x,dx=0; s[x]!=x; dx+=d[x], x=s[x]);
	for (y0=y,dy=0; s[y]!=y; dy+=d[y], y=s[y]);
	s[x0]=x;
	d[x0]=dx;
	s[y0]=y;
	d[y0]=dy;
	if (x<y) {
	  s[y] = x;
	  d[y] = z + dx - dy;
	} else if (x>y) {
	  s[x] = y;
	  d[x] = -z - dx + dy;
	} else continue;
	
      } else { // diff

	for (x0=x,dx=dy=0; s[x]!=x; dx+=d[x], x=s[x]);
	for (rx=x, x=x0, ex=dx; s[x]!=x; x=sx) {
	  sx=s[x];
	  s[x]=rx;
	  tmp=d[x];
	  d[x]=ex;
	  ex-=tmp;
	}
	for (y0=y; s[y]!=y; dy+=d[y], y=s[y]);
	v=-dx+dy;
	if (x!=y) {
	  fputs("UNKNOWN\n", stdout);
	} else {
	  i=30;
	  if (v>0)  for (; v>0; buf[--i]='0'+v%10,v/=10);
	  else if (v<0) {
	    for (; v<0; buf[--i]='0'-v%10,v/=10);
	    buf[--i] = '-';
	  } else  buf[--i] = '0';
	  fputs(buf+i, stdout);
	}
      }
    }
  }
  return 0;
}

