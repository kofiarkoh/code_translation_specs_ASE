#include<stdio.h>
int main(void){
  int a,b,c,k;
  scanf("%d%d%d%d",&a,&b,&c,&k);
  for(int i=0;i<k;i++){
      if(a>=b){
        b*=2;
      }
      else if(b>=c)c*=2;
    
     if(c>b&&b>a){
      printf("Yes");
      return 0;
    }
}
  printf("No");
  return 0;
}
