#include<stdio.h>
int gcd(int a,int b){return b?gcd(b,a%b):a;}
int main(){
  int n,a,b;
  long long x=0,y=1;
  scanf("%d",&n);
  while(n--){
    scanf("%d %d",&a,&b);
    if(a==1){
      b*=y;
      x+=b;
    }
    if(a==2){
      b*=y;
      x-=b;
    }      
    if(a==3)x*=b;
    if(a==4)y*=b;//printf("%f\n",x);
    b=gcd(x,y);
    x/=b;
    y/=b;
  }
  printf("%lld\n",x/y);
  return 0;
}