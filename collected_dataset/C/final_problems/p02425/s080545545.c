#include <stdio.h>

int main()
{
  int nq, q, i, j, count = 0;
  char buf[32];
  buf[31]='\0';
  buf[30]='\n';
  char array[64] = {0};
  unsigned long val, r;
  fgets(buf, 32, stdin);
  char *p = buf;
  for (nq=0; *p>='0'; p++)  nq = nq * 10 + *p - '0';
  while (nq-->0) {
    fgets(buf, 32, stdin);
    q = buf[0]-'0';
    switch (q) {
    case 0: // test(i)
      i = (buf[3]=='\n') ? buf[2]-'0' : (buf[2]-'0')*10+buf[3]-'0';
      fputs(array[i]?"1\n":"0\n", stdout);
      break;
    case 1: // set(i)
      i = (buf[3]=='\n') ? buf[2]-'0' : (buf[2]-'0')*10+buf[3]-'0';
      if (array[i]==0) {
	array[i] = 1;
	count++;
      }
      break;
    case 2: // clear(i)
      i = (buf[3]=='\n') ? buf[2]-'0' : (buf[2]-'0')*10+buf[3]-'0';
      if (array[i]) {
	array[i] = 0;
	count--;
      }
      break;
    case 3: // clip(i)
      i = (buf[3]=='\n') ? buf[2]-'0' : (buf[2]-'0')*10+buf[3]-'0';
      if (array[i]) {
	array[i] = 0;
	count--;
      } else {
	array[i] = 1;
	count++;
      }
      break;
    case 4: // all
      fputs(count==64?"1\n":"0\n", stdout);
      break;
    case 5: // any
      fputs(count>0?"1\n":"0\n", stdout);
      break;
    case 6: // none
      fputs(count?"0\n":"1\n", stdout);
      break;
    case 7: // count
      if (count==0) {
	fputs("0\n", stdout);
      } else {
	int v = count;
	for (j=29; v>0; j--, v/=10)   buf[j] = '0' + (v%10);
	fputs(buf+j+1, stdout);
      }
      break;
    case 8: // val
      if (count==0) {
	fputs("0\n", stdout);
      } else {
	val = array[0];
	r = 2;
	for (int j=1; j<64; j++,r<<=1)
	  val += array[j]*r;
	for (j=29; val>0; j--, val/=10)   buf[j] = '0' + (val%10);
	fputs(buf+j+1, stdout);
      }
    }
  }
  return 0;
}

