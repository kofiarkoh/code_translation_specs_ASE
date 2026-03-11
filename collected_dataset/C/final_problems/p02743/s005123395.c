#include<stdio.h>
#include<math.h>
int main (){
  long a,b,c;
  scanf("%ld%ld%ld",&a,&b,&c);
  if((double)(a+b)<(double)(c-2*sqrt(a*b))){
    printf("Yes\n");
  }else{
    printf("No\n");
  }
  return 0;
}