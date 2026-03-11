#include <stdio.h>

int main(){

  int n, a, ans, i, c;

  scanf("%d", &n);

  for(i=0;i<n;i++){
    c=0;
    scanf("%d", &a);
    while(a%2==0 && a>0){
      c++;
      a/=2;
    }
    if(ans>c || i==0) ans=c;
  }
  printf("%d\n", ans);
  return 0;
}