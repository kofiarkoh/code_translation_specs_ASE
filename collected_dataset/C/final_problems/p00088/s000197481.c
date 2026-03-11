#include <stdio.h>
int main()
{
  unsigned char *a = "\n ',-.?ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  unsigned char *b[] = {
    "101","101","000000","000011","10010001","010001","000001","100101",
    "10011010","0101","0001","110","01001","10011011","010000","0111",
    "10011000","0110","00100","10011001","10011110","00101","111",
    "10011111","1000","00110","00111","10011100","10011101","000010",
    "10010010","10010011","10010000"
  };
  unsigned char *c = "ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";
  unsigned char d[4096];
  unsigned char *p;
  int i,j,k;
  i = 0;
  while((k = getc(stdin)) != '\n'){
    j = 0;
    while(*(a + j) != k) j++;
    p = b[j];
    while(*p) d[i++] = *p++;
  }
  while(i % 5) d[i++] = '0';
  d[i] = '\0';
  i = 0;
  while(d[i]) {
    k = 0;
    j = 4;
    do {
      k |= (d[i++] == '1') << j;
    } while(j--);
    putc(*(c + k),stdout);
  }
  putc('\n',stdout);
  return 0;
}