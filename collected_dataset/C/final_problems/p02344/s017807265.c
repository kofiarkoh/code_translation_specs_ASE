#include <stdio.h>

int main()
{
  int i, n, v, x, y, z, dx, dy;
  char *p, buf[32];
  buf[31]=0;
  buf[30]='\n';
  fgets(buf, 32, stdin);
  for (n=0,p=buf; *p!=' '; n=n*10+*(p++)-'0'); 
  int s[n], d[n];
  for (i=0; i<n; s[i]=i,d[i++]=0);
  while (fgets(buf, 32, stdin)) {
    for (x=0,p=buf+2; *p!=' '; x=x*10+*(p++)-'0');
    if (buf[0]=='0') { // relate x y z
      for (y=0,p++; *p!=' '; y=y*10+*(p++)-'0');
      for (z=0,p++; *p!='\n'; z=z*10+*(p++)-'0');
      for (dx=0; s[x]!=x; dx+=d[x],x=s[x]);
      for (dy=0; s[y]!=y; dy+=d[y],y=s[y]);
      if (x==y) continue;
      if (x<y) {
	s[y] = x;
	d[y] = dx + z - dy;
      }	else {
	s[x] = y;
	d[x] = dy - z - dx;
      }
    } else { // diff
      for (y=0,p++; *p!='\n'; y=y*10+*(p++)-'0');
      for (v=0; s[x]!=x; v-=d[x], x=s[x]);
      for (;s[y]!=y; v+=d[y], y=s[y]);
      if (x!=y) {
	putc_unlocked('?', stdout);
	putc_unlocked('\n', stdout);
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
  return 0;
}

