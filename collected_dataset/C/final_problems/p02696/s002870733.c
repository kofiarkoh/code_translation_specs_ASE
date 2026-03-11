#include<stdio.h>
long long int gcd(long long int a,long long int b){
  long long int i;
  while(a!=0 && b!=0){
    i=b;
    b=a%b;
    a=i;
  }
  return a;
}


int main(){
  long long a,b,n,x,i,max=0;
  scanf("%lld%lld%lld",&a,&b,&n);
  if(a%b==0){
    if(b-1<=n)
      printf("%lld",b-1);
    else
      printf("%lld",n);
    return 0;
  }
  x=b/gcd(a,b);
  if(x<=n)
    printf("%lld\n",a/gcd(a,b));
  else{
    for(i=0;i<=n;i++)
      if(i%b>max)
        max=i%b;
    printf("%lld\n",(max*a)/b);
  }
  return 0;
}
