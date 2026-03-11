#include <stdio.h>

int main()
{
  const int pow2[] = { 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824};
  
  char buf[64];
  buf[63]=0;
  buf[62]='\n';
  int i, j, k, m, n, o, v;
  fgets(buf, 64, stdin);
  n = buf[2]=='\n' ? (buf[0]-'0')*10+buf[1]-'0' : buf[0]-'0';
  fgets(buf, 64, stdin);
  if (buf[0]=='0') {
    fputs("0:\n",stdout);
    return 0;
  }

  char *p;
  if (buf[2]==' ') {
    k = (buf[0]-'0')*10+buf[1]-'0';
    p = buf+3;
  } else {
    k = buf[0]-'0';
    p = buf+2;
  }
  int b[28], d[28];
  b[0] = 0;  
  for (i=0; *p!='\n'; p++) {
    if (*p == ' ')
      b[++i] = *(++p)-'0';
    else
      b[i] = b[i] * 10 + *p - '0';
  }

  for (i=0;i<k;i++)    d[i]=pow2[b[i]];
  m = pow2[k];
  for (i=0; i<m; i++) {
    v = 0;
    o = 61;
    for (j=k; j>=0; j--) {
      if (i & pow2[j]) {
	v += d[j];
	int ov=b[j];
	if (ov==0) {
	  buf[o--]='0';
	}
	for (; ov>0; o--, ov/=10)   buf[o] = '0' + (ov%10);
	buf[o--]=' ';
      }
    }
    buf[o--] = ':';
    if (v==0)     buf[o--]='0';
    else {
      for (; v>0; o--, v/=10)
	buf[o] = '0' + (v%10);
    }
    fputs(buf+o+1, stdout);
  }

  return 0;
}

