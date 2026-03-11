#include<stdio.h>

int main(){
  int n,k;
  unsigned long count=0;
  unsigned long abs_s=0;
  unsigned long abs_t=0, abs_u=0;
  scanf("%d %d", &n, &k);
  
  for(int a=1;a<n+1;a++){
    if((2*a)%k==0){
      abs_s++;
      if(a%k==0)
        abs_t++;
    }
  }

  abs_u = abs_s - abs_t;

  count = abs_s + (abs_u*(abs_u-1))*3 + (abs_t*(abs_t-1))*3 + (abs_u*(abs_u-1)*(abs_u-2)) + (abs_t*(abs_t-1)*(abs_t-2));

  printf("%lu\n", count);

  return 0;
}