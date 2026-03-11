#include <stdio.h>
#include <stdlib.h>

int main()
{
  int i, j, k, ii, c, d, n, m;
  char buf[16] = {0};
  buf[15] = '\0';
  buf[14] = '\n';
  fscanf(stdin, "%d\n", &n);
  int *a = (int*)malloc(sizeof(int)*n);
  
  int sep = '\n';
  char *line = NULL;
  size_t linecap = 0;
  ssize_t linelen;
  getline(&line, &linecap, stdin);
  
  a[0] = 0;  
  i=0;
  for (char *p=line; *p!='\n'; p++) {
    if (*p < '0') {
      i++;
      p++;
      a[i] = *p-'0';
    } else {
      a[i] = a[i] * 10 + *p - '0';
    }
  }
  
  fscanf(stdin, "%d\n", &m);
  getline(&line, &linecap, stdin);
  char *p=line;
  i = j = 0;
  
  int b;
  for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
  p++;
  
  if (a[i]<=b) {
    c = a[i++];
  } else {
    c = b;
    j++;
    for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
    p++;
  }
  
  d = c;
  if (d==0) {
    fputs("0\n", stdout);
  } else {
    for (ii=13; d>0; ii--, d/=10)   buf[ii] = '0' + (d%10);
    fputs(buf+ii+1, stdout);
  }
  
  while (i<n && j<m) {
    if (c==a[i]) {
      i++;
      continue;
    }
    if (c==b) {
      j++;
      for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
      p++;
      continue;
    }
    
    if (a[i]<=b) {
      c = a[i++];
    } else {
      c = b;
    }
    //    printf("%d\n", c);
    d = c;
    for (ii=13; d>0; ii--, d/=10)   buf[ii] = '0' + (d%10);
    fputs(buf+ii+1, stdout);
  }


  while (i<n && j<m) {
    if (c==a[i]) {
      i++;
      continue;
    }
    if (c==b) {
      j++;
      for (b = 0; ; b = b * 10 + *(p++) - '0');
      p++;
      continue;
    }
    
    if (a[i]<=b) {
      c = a[i++];
    } else {
      c = b;
    }

    for (ii=13; d>0; ii--, d/=10)   buf[ii] = '0' + (d%10);
    fputs(buf+ii+1, stdout);
      
  }
  
  while (i<n) {
    if (c==a[i]) {
      i++;
      continue;
    }
    c = a[i++];
    d = c;
    for (ii=13; d>0; ii--, d/=10)   buf[ii] = '0' + (d%10);
    fputs(buf+ii+1, stdout);

  }
  
  while (j<m) {
    if (c==b) {
      j++;
      for (b = 0; *p>='0'; b = b * 10 + *(p++) - '0');
      p++;
      continue;
    }
    c = b;
    d = b;
    for (ii=13; d>0; ii--, d/=10)   buf[ii] = '0' + (d%10);
    fputs(buf+ii+1, stdout);

  }
  return 0;
}

