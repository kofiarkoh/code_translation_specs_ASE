#include <stdio.h>

int pow10(int n){
  int ret=1,i;
  for(i=0;i<n;i++)
    ret*=10;
  return ret;
}

int solve(int s){
  int n,ret=0,i;
  n=s*s/100;
  for(i=0;i<4;i++){
    ret+=(n%10)*pow10(i);
    n/=10;
  }
  return ret;
}

int main(void){
  int d,i,j,s;
  scanf("%d",&d);
  i=1;
  while(d--){
    scanf("%d",&s);
    printf("Case %d:\n",i);
    for(j=0;j<10;j++){
      s=solve(s);
      printf("%d\n",s);
    }
    i++;
  }
  return 0;
}