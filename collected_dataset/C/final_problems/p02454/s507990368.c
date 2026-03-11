#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int pos;
  int val;
} Val;

int main()
{
  int i, j, k, n, m, q, v, lb, ub;
  char buf[32];
  fscanf(stdin, "%d\n", &n);
  Val *a = (Val*)malloc(sizeof(Val)*n);
  int alen=0;

  char *line=NULL;
  size_t linecap = 0;
  getline(&line, &linecap, stdin);
  a[alen].pos = 0;
  a[alen++].val = atoi(line);
  line = index(line+1, ' ');
  for (i=1; i<n; i++) {
    v = atoi(line);
    if (a[alen-1].val<v) {
      a[alen].pos = i;
      a[alen++].val = v;
    }
    line = index(line+1, ' ');
  }
  
  int begin, end, mid;
  fscanf(stdin, "%d\n", &q);
  for (i=0; i<q; i++) {
    fgets(buf, 32, stdin);
    k = atoi(buf);
    begin=0;
    end=alen-1;
    int undecided_lb=1;
    while (begin<=end) {
      mid=(begin+end)/2;
      if (a[mid].val==k) {
	lb =  a[mid].pos;
	undecided_lb=0;
      }
      if (a[mid].val>k) {
        end = mid - 1;
      } else begin = mid + 1;
    }
    if (begin>=alen) {
      if (undecided_lb) lb = n;
      ub = n;
    } else {
      if (undecided_lb) lb = a[begin].pos;
      ub = a[begin].pos;
    }
    int b=lb;
    int k;
    if (b==0) {
      putc_unlocked('0', stdout);
    } else {
      if (b<0) {
	putc_unlocked('-', stdout);
	b=-b;
      }
      for (j=1; j*10<=b; j*=10);
      k=j;
      for (;j>0; j/=10) {
	putc_unlocked('0'+b/j, stdout);
	b %= j;
      }
    }
    putc_unlocked(' ', stdout);
    if (lb==ub) {
      if (ub==0) {
	putc_unlocked('0', stdout);
      } else {
	if (ub<0) {
	  putc_unlocked('-', stdout);
	  ub=-ub;
	}
	j=k;
	for (;j>0; j/=10) {
	  putc_unlocked('0'+ub/j, stdout);
	  ub %= j;
	}
      }
    } else {
      if (ub==0) {
	putc_unlocked('0', stdout);
      } else {
	if (ub<0) {
	  putc_unlocked('-', stdout);
	  ub=-ub;
	}
	for (j=1; j*10<=ub; j*=10);
	for (;j>0; j/=10) {
	  putc_unlocked('0'+ub/j, stdout);
	  ub %= j;
	}
      }
    }
    putc_unlocked('\n', stdout);
  }
  
  return 0;
}

