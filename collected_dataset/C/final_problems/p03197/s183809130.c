#include<stdio.h>
int main(){
  int n,a,m=0;
  scanf("%d",&n);
  while(n--){
    scanf("%d",&a);
    if(a%2)m=1;
  }
  printf("%s\n",m?"first":"second");
  return 0;
}
