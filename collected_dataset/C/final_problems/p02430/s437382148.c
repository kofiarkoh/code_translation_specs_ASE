#include <stdio.h>

const int pow2[] = { 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144};

int num[18], u[18];
int n, k;

void run(int m)
{
  static char buf[64] = {0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0, 0,0,'\n',0};
  //  buf[63]=0;
  //  buf[62]='\n';

  if (m==k) {
    int d, v, o=61;
    d = 0;
    for (int i=0; i<k; i++) {
      v = num[i];
      d += pow2[v];
      if (v==0) {
	buf[o--]='0';
      }
      for (; v>0; o--, v/=10)   buf[o] = '0' + (v%10);
      buf[o--]=' ';
    }
    buf[o--] = ':';
    for (; d>0; o--, d/=10)
      buf[o] = '0' + (d%10);
    fputs(buf+o+1, stdout);
    
  } else {
    for (int i=0; i<n; i++) {
      if (m>0 && num[m-1]<i) continue;
      if (u[i]) continue;
      num[m] = i;
      u[i]=1;
      run(m+1);
      u[i]=0;
    }
  }
}

int main()
{
  char buf[8];
  char *p=buf;
  fgets(p, 8, stdin);
  if (p[2]==' ') {
    n = (p[0]-'0')*10+p[1]-'0';
    p += 3;
  } else {
    n = p[0]-'0';
    p += 2;
  }
  k = (*(p+2)=='\n') ? (*p-'0')*10+*(p+1)-'0' : *p-'0';

  run(0);

  return 0;
}

