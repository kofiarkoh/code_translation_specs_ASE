#include<stdio.h>
int main(){
  int a,b,c,n,d=0;
  scanf("%d %d %d",&n,&a,&b);
  while(n--){
    scanf("%d",&c);
    if(c<a||b<=c)d++;
  }
  printf("%d\n",d);
  return 0;
}
