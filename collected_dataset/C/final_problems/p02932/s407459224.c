#include<stdio.h>
#define mod 1000000007

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

long long factorial[4194304];
long long invfact[4194304];

void cfact(){
  long long i;
  factorial[0]=1;
  factorial[1]=1;
  for(i=2;i<4194304;i++){
    factorial[i]=factorial[i-1]*i;
    factorial[i]%=mod;
  }
  invfact[4194303]=modular_inverse(factorial[4194303]);
  for(i=4194302;i>=0;i--){
    invfact[i]=invfact[i+1]*(i+1);
    invfact[i]%=mod;
  }
}

long long calcnCr(long long n,long long k){
  if(k<0 || n-k<0){return 0;}
  return (factorial[n]*((invfact[k]*invfact[n-k])%mod))%mod;
}

//0 <= A_1 + A_2 + ... + A_n <= s
//A_1,A_2,...,A_m <= g
long long hj(long long n,long long m,long long s,long long g){
  long long i,t,res=0,k=1,c;
  if(s<0 || g<0){return 0;}
  //Rule v : A_v <= g
  for(i=0;i<=m;i++){
    //violating i rules(the others are no matter)
    t=s-i*(g+1);
    if(t<0){break;}
    c=calcnCr(m,i);
    c*=calcnCr(t+n,n);
    c%=mod;
    if(k==1){res+=c;}
    else{res+=(mod-c);}
    res%=mod;
    k*=-1;
  }
  return res;
}

//0 <= A_1 + A_2 + ... + A_n <= r
//m-th greater != (m+1)-th greater
long long neq(long long n,long long m,long long r){
  long long i,j,res=0,c,k;
  for(i=0;i<=r;i++){
    //(m+1)-th greater == i
    res+=hj(n,n-m,r-(i+1)*m,i);
    res%=mod;
    res+=(mod-hj(n,n-m,r-(i+1)*m,i-1));
    res%=mod;
  }
  res*=calcnCr(n,m);
  res%=mod;
  return res;
}

//0 <= A_1 + A_2 + ... + A_n <= r
//m-th greater == (m+1)-th greater
long long rside(long long n,long long m,long long r){
  return (mod+calcnCr(r+n,n)-neq(n,m,r))%mod;
}

//l <= A_1 + A_2 + ... + A_n <= r
//m-th greater == (m+1)-th greater
long long calc(long long n,long long m,long long l,long long r){
  return (mod+rside(n,m,r)-rside(n,m,l-1))%mod;
}

int main(){
  cfact();
  long long n,m,l,r;
  scanf("%lld%lld%lld%lld",&n,&m,&l,&r);
  printf("%lld\n",calc(n,m,l,r));
}