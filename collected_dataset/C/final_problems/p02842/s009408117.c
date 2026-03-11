#include<stdio.h>
#include<math.h>

int main(void){
  double n,n1,a,b;
  int c;
  
  scanf("%lf",&n);
  n1 = (double)n / 1.08;
  a = ceil(n1);
  b = floor(n1);
  
  if(a != b){
      if(floor(a * 1.08) == n)
        printf("%d\n",(int)a);
      else if(floor(b * 1.08) == n)
        printf("%d\n",(int)b);
      else
        printf(":(\n");
  }  
  else
    printf("%d\n",(int)n1);
  
  return 0;
}