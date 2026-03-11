#include <stdio.h>
int main(void){
  int n, i, j;
  scanf("%d", &n);
  int p[n], exp=0, sum=0;
  for(i=0; i<n; i++){
    scanf("%d", &p[i]);
    sum+=p[i];
    if(exp<=p[i])
      exp=p[i];
  }
  sum=sum-0.5*exp;
  printf("%d\n", sum);
  return 0;
}