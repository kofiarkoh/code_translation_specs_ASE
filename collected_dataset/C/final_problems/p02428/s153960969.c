#include <stdio.h>

int main()
{
  int i, j, n;
  unsigned int pow2[] = {1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144};
  char buf[128];
  buf[127] = 0;
  buf[126] = ':';
  int b[19];
  fscanf(stdin, "%d\n", &n);
  unsigned int u, v, val=1;
  fgets(buf, 120, stdin);
  b[0] = 0;  
  i=0;
  for (char *p=buf; *p!='\n'; p++) {
    if (*p == ' ') {
      i++;
      p++;
      b[i] = *p-'0';
    } else {
      b[i] = b[i] * 10 + *p - '0';
    }
  }
  unsigned int c=0;
  for (i=0; i<b[0]; i++)  c |= pow2[b[i+1]];
  
  if (b[0]==0)  fputs("0:\n", stdout);
  //  for (i=0; i<n; i++) val <<= 1;
    val = pow2[n];
  for (u=1; u<val; u++) {
    v = u;
    if ((u & c) != c)  continue;
    for (j=125; v>0; j--, v/=10)   buf[j] = '0' + (v%10);
    fputs(buf+j+1, stdout);
    v = u;
    for (i=0; i<n; i++) {
      if (v%2) {
	putc_unlocked(' ', stdout);
	if (i>=10)	putc_unlocked('0' + i/10, stdout);
	putc_unlocked('0' + i%10, stdout);
      }
      v>>=1;
    }
    putc_unlocked('\n',stdout);
  }
  return 0;
}

