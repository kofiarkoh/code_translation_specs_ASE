#include <stdio.h>

int sqr(int x){
  return x*x;
}

int main(void){

  int n,a[3],l,i;
  scanf("%d",&n);

  for(i=0;i<n;i++){
    scanf("%d %d %d",&a[0],&a[1],&a[2]);
    l=a[0];
    if(l < a[1]) l=a[1];
    if(l < a[2]) l=a[2];

    if(2*sqr(l)==sqr(a[0])+sqr(a[1])+sqr(a[2]))printf("YES\n");
    else printf("NO\n");

  }

  return 0;
}