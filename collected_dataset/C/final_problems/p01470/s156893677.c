#include<stdio.h>
int main(){
  int n,i;
  int a,b;
  long double s=0;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d %d",&a,&b);
    if(a==1)s+=b;
    if(a==2)s-=b;
    if(a==3)s*=b;
    if(a==4)s/=b;
  }
  printf("%d\n",(int)s);
  return 0;
}