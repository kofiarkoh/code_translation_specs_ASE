#include<stdio.h>
#define mod 1000000007

long long factorial[524288];

void cfact(){
    long long i;
    factorial[0]=1;
    factorial[1]=1;
    for(i=2;i<524288;i++){
        factorial[i]=factorial[i-1]*i;
        factorial[i]%=mod;
    }
}

long long power(long long a,long long b){
  long long x=1,y=a;
  while(b>0){
    if(b&1ll){
      x=(x*y)%mod;
    }
    y=(y*y)%mod;
    b>>=1;
  }
  return x%mod;
}
 
long long modular_inverse(long long n){
	return power(n,mod-2);
}

long long calcnCr(long long n,long long k){
	return (factorial[n]*(modular_inverse(factorial[k])*modular_inverse(factorial[n-k])%mod))%mod;
}

long long calc(long long a,long long b,long long c,long long d){
  long long n=a+b+c+d,r;
  r=calcnCr(n,a);
  r*=calcnCr(n-a,b);r%=mod;
  r*=calcnCr(n-a-b,c);
  return r%mod;
}

int main(){
  cfact();
  long long n,a,b,c,r=0,w;
  long long alpha,beta,gamma,delta,sigma;
  scanf("%lld%lld%lld%lld",&n,&a,&b,&c);
  if(b%2==1){printf("0\n");return 0;}
  beta=b/2;
  for(alpha=0;alpha<=a;alpha++){
    gamma=a-alpha;
    for(delta=0;3*delta<=c;delta++){
      //printf("%lld %lld %lld %lld\n",alpha,beta,gamma,delta);
      w=calc(alpha,beta,gamma,delta);
      sigma=c-3*delta-gamma;
      if(sigma<0){continue;}
      if(beta==0){
        if(sigma==0){
          r+=w;r%=mod;
        }
      }
      else{
        r+=((w*calcnCr(sigma+(beta-1),beta-1))%mod);r%=mod;
      }
    }
  }
  printf("%lld\n",r);
  return 0;
}