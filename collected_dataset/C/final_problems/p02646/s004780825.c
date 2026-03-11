#include<stdio.h>
signed main(){
  int A,V,B,W,T;
  scanf("%d %d %d %d %d",&A,&V,&B,&W,&T);
  if(A+V*T>=B+W*T){
    printf("YES\n");
  }else{
    printf("NO\n");
  }
  return(0);
}