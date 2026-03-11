#include <stdio.h>
int main(){
  int i,n,a,b=0,c=0;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    if(a==b){
      c++;
      b=0;
    }else b=a;
  }
  printf("%d",c);
  return 0;
}