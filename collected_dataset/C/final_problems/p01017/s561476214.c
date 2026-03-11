#include <stdio.h>

typedef long long ll;

int main(void)
{
  int h,w,i,j,ii,jj;
  int hh,ww;
  int a[55][55];
  int b[55][55],c[55][55];
  scanf("%d%d",&h,&w);
  for( i = 0; i < h; i++ ) {
    for( j = 0; j < w; j++ ) {
      scanf("%d",a[i]+j);
    }
  }
  for( i = 0; i < h; i++ ) {
    for( j = 0; j < w; j++ ) {
      scanf("%d",b[i]+j);
    }
  }
  scanf("%d%d",&hh,&ww);
  for( i = 0; i < hh; i++ ) {
    for( j = 0; j < ww; j++ ) {
      scanf("%d",c[i]+j);
    }
  }
  int res = -999999999;
  int dame = 1;
  for( i = 0; i <= h-hh; i++ ) {
    for( j = 0; j <= w-ww; j++ ) {
      int s = 0;
      int ok = 1;
      for( ii = 0; ii < hh; ii++ ) {
        for( jj = 0; jj < ww; jj++ ) {
          s += a[i+ii][j+jj];
          if( b[i+ii][j+jj] != c[ii][jj] ) ok = 0;
        }
      }
      if( ok ) {
        dame = 0;
        if( res < s ) res = s;
      }
    }
  }
  if( dame ) puts("NA");
  else printf("%d\n",res);
  return 0;
}  
