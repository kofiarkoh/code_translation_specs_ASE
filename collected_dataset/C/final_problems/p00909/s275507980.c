#include <stdio.h>
#include <stdlib.h>

int main()
{
  int i, n, q, v, x, y, z, dx, dy;
  char *p, buf[32];
  buf[31]=0;
  buf[30]='\n';
  int *s, *d;
  while (1) {
    fgets(buf, 32, stdin);
    for (n=0,p=buf; *p!=' '; n=n*10+*(p++)-'0'); 
    for (q=0,p++; *p!='\n'; q=q*10+*(p++)-'0'); 
    if (n==0 && q==0) break;
    s = (int*)malloc(sizeof(int)*n);
    d = (int*)malloc(sizeof(int)*n);
    for (i=0; i<n; s[i]=i,d[i++]=0);
    for (int t=0; t<q; t++) {
      fgets(buf, 32, stdin);
      for (x=0,p=buf+2; *p!=' '; x=x*10+*(p++)-'0');
      for (y=0,p++; *p>='0'; y=y*10+*(p++)-'0');
      x--;
      y--;
      if (buf[0]=='!') { // relate x y z
	for (z=0,p++; *p!='\n'; z=z*10+*(p++)-'0');
	for (dx=0; s[x]!=x; dx+=d[x],x=s[x]);
	for (dy=0; s[y]!=y; dy+=d[y],y=s[y]);
	if (x<y) {
	  s[y] = x;
	  d[y] = dx + z - dy;
	}	else if (x>y) {
	  s[x] = y;
	  d[x] = dy - z - dx;
	} else continue;
      } else { // diff
	for (v=0; s[x]!=x; v-=d[x], x=s[x]);
	for (;s[y]!=y; v+=d[y], y=s[y]);
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
    free(s);
    free(d);
  }
  return 0;
}

