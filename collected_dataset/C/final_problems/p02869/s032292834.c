#include<stdio.h>

int main(){
  long long n,k,sa,sb,sc;
  long long a,b,c;
  scanf("%lld%lld",&n,&k);
  sa=(((k)+(k+n-1))*n)/2;
  sb=(((k+n)+(k+2*n-1))*n)/2;
  sc=(((k+2*n)+(k+3*n-1))*n)/2;
  if(sa+sb>sc){printf("-1\n");return 0;}
   a=(n/2);
   b=-1;
   for(c=0;c<n;c++){
    a--;
    if(a<0){a=n-1;}
    b+=2;
    if(b>=n){
      if(n%2==0){b--;}
      b%=n;
    }
    printf("%lld %lld %lld\n",k+a,k+n+b,k+2*n+c);
  }
}