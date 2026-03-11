#include <stdio.h>

unsigned long a1[2 * 100000];
unsigned long a2[2 * 100000];
unsigned long pos = 0;

unsigned long adddata(unsigned long val)
{
    a1[pos++] = val;

    for( unsigned long loop = 0; loop < pos - 1; loop ++ ) {
      if( a1[loop] == val ) {
        pos = loop;
        break;
      }
    }

    return pos;
 }


int main(void)
{
  unsigned long n;
  unsigned long v;
  unsigned long loop;
  
  scanf("%ld", &n);
  scanf("%ld", &v);
  
  for( loop = 0; loop < n; loop ++ ) {
	   scanf("%ld", &v);
       a2[loop] = v;
       adddata(v);
  }
  for( loop = 0; loop < n; loop ++ ) {
       adddata(a2[loop]);
  }

  if( pos == 0 ) {
    printf( "\n");
  }
  else {
    for( loop = 0; loop < pos-1; loop ++ ) {
        printf( "%ld ", a1[loop]);
    }
    printf( "%ld\n", a1[pos-1]);
  } 
  
  
  return 0;
}
