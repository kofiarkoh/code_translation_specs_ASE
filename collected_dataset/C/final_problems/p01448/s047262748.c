#include<stdio.h>
int a,b,i,c[100010];

int main(){
  for(scanf("%d",&i);~scanf("%d%d",&a,&b);)
    for(;b>=a;a++)
      c[a]++;
  for(a=100001;a;){
    --a;
    if(c[a+1]>=a){
      printf("%d\n",a);
      break;
    }
  }
}