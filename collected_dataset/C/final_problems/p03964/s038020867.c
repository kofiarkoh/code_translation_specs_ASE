#include <stdio.h>

int main(){
  int n,i,j,anow,tnow,anew,tnew;
  int t[1010],a[1010];
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d %d",&t[i],&a[i]);
  }
  anow = a[0];
  tnow = t[0];
  for(i=0;i<(n-1);i++){
    j=1;
    anew = tnew = 0;
    while(anew < anow || tnew < tnow){
      anew = j*a[i+1];
      tnew = j*t[i+1];
      j++;
    }
    anow = anew;
    tnow = tnew;
  }
  printf("%d",tnow+anow);
}