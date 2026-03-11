#include <stdio.h>
#include <stdlib.h>

int main()
{
  int i, j, k, b, d, n, m;

  char buf[16] = {0};
  char *p;
  buf[15] = '\0';
  buf[14] = '\n';
  char *line = NULL;
  size_t linecap = 0;
  fgets(buf, 15, stdin);
  n = atoi(buf);
  int *a = (int*)malloc(sizeof(int)*n);
  getline(&line, &linecap, stdin);
  a[0] = 0;  
  i=0;
  for (p=line; *p!='\n'; p++) {
    if (*p == ' ') {
      i++;
      p++;
      a[i] = *p-'0';
    } else {
      a[i] = a[i] * 10 + *p - '0';
    }
  }
  fgets(buf, 15, stdin);
  m = atoi(buf);
  
  getline(&line, &linecap, stdin);
  p=line;
  i = j = 0;
  for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
  p++;

  while (i<n-1) {
    if (a[i]<b) {
      d=a[i++];
      if (d==0) {
	fputs("0\n", stdout);
      } else {
	for (k=13; d>0; k--, d/=10)   buf[k] = '0' + (d%10);
	fputs(buf+k+1, stdout);
      }
    } else if (a[i]>b) {
      if (j<m-1) {
	for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
	p++;
	j++;
      } else {
	d=a[i++];
	for (k=13; d>0; k--, d/=10)   buf[k] = '0' + (d%10);
	fputs(buf+k+1, stdout);
      }
    } else {
      i++;
      if (j<m-1) {
	for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
	p++;
	j++;
      }
    }
  }
  for (; j<m; j++) {
    if (a[i]==b) break;
    for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
    p++;
  }
  if (j==m) {
    d = a[i];
    if (d==0) {
      fputs("0\n", stdout);
    } else {
      for (k=13; d>0; k--, d/=10)   buf[k] = '0' + (d%10);
      fputs(buf+k+1, stdout);
    }
  }
  
  return 0;
}


