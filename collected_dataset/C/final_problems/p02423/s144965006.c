#include <stdio.h>

int main()
{
  unsigned int b, n, m;
  fscanf(stdin, "%u", &n);
  m=n;
  int i=0;
  char c;
  char buf[33];
  buf[32]=0;
  char buf2[33];
  buf2[32]=0;
  buf2[31]='0';
  char buf3[33];
  buf3[0]='0';
  for (b = 2147483648U; b>0; b>>=1) {
    printf("%d", n/b);
    buf[i]='0'+1-n/b;
    if(i>0) buf2[i-1]='0' + n/b;
    buf3[i+1]='0' + n/b;
    i++;
    n%=b;
  }
  buf3[32]=0;
  printf("\n");
  puts(buf);
  puts(buf2);
  puts(buf3);

  return 0;
}

