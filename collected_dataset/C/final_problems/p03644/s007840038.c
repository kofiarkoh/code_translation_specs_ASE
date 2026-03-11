#include <stdio.h>

int main(void) {
  int i;
  int n;
  int ans;
  int a;
  int cnt=0;
  int cnt2=-1;


  scanf("%d",&n );

  for ( i = 1; i < n+1; i++) {
    a=i;
 
    for ( cnt = 0; a%2==0; cnt++) {
      a=a/2;
    }
    if (cnt2<cnt) {
      ans=i;
      cnt2=cnt;
    }
  }
  printf("%d",ans );


  return 0;
}