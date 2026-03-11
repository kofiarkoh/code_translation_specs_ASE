#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int pos;
  int val;
} Val;

int main()
{
  int i, j, k, n, m, q, v;
  char buf[32];
  fscanf(stdin, "%d", &n);
  Val *a = (Val*)malloc(sizeof(Val)*n);
  int alen=0;
  fscanf(stdin, "%d", &v);
  a[alen].pos=0;
  a[alen++].val=v;
  for (i=1; i<n; i++) {
    fscanf(stdin, "%d", &v);
    if (a[alen-1].val<v) {
      a[alen].pos = i;
      a[alen++].val = v;
    }
  }
  int begin, end, mid;
  fscanf(stdin, "%d\n", &q);
  for (i=0; i<q; i++) {
    fgets(buf, 32, stdin);
    k = atoi(buf);
    begin=0;
    end=alen-1;
    while (begin<=end) {
      mid=(begin+end)/2;
      if (a[mid].val==k) {
	int b = a[mid].pos;
	if (b==0) {
	  putc_unlocked('0', stdout);
	} else {
	  if (b<0) {
	    putc_unlocked('-', stdout);
	    b=-b;
	  }
	  for (j=1; j*10<=b; j*=10);
	  for (;j>0; j/=10) {
	    putc_unlocked('0'+b/j, stdout);
	    b %= j;
	  }
	}
	putc_unlocked('\n', stdout);

        goto next;
      }
      if (a[mid].val>=k) {
        end = mid - 1;
      } else begin = mid + 1;
    }
    if (begin>=alen) {
      int b = n;
      if (b==0) {
	putc_unlocked('0', stdout);
      } else {
	if (b<0) {
	  putc_unlocked('-', stdout);
	  b=-b;
	}
	for (j=1; j*10<=b; j*=10);
	for (;j>0; j/=10) {
	  putc_unlocked('0'+b/j, stdout);
	  b %= j;
	}
      }
      putc_unlocked('\n', stdout);
    } else {
      int b = a[begin].pos;
      if (b==0) {
	putc_unlocked('0', stdout);
      } else {
	if (b<0) {
	  putc_unlocked('-', stdout);
	  b=-b;
	}
	for (j=1; j*10<=b; j*=10);
	for (;j>0; j/=10) {
	  putc_unlocked('0'+b/j, stdout);
	  b %= j;
	}
      }
      putc_unlocked('\n', stdout);
    }
  next:
    ;
  }
  
  return 0;
}

