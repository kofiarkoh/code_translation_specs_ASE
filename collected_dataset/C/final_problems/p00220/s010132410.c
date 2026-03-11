#include<stdio.h>
int main(){
  double n;
  char a[12];
  int m,i;
  while(1){
    scanf("%lf",&n);
    if(n<0)break;
    n*=16;
    m=n;
    if(m-n){
      printf("NA\n");
      continue;
    }
    for(i=12;i;i--){
      a[i-1]=m%2+'0';
      m/=2;
    }
    if(m){
      printf("NA\n");
      continue;
    }
    for(i=0;i<12;i++){
      if(i==8)printf(".");
      printf("%c",a[i]);
    }
    printf("\n");
  }
  return 0;
}