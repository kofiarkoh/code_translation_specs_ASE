#include<stdio.h>
int mbk(long long a,int n,int m){return n?(mbk((a*a)%m,n/2,m)*(n%2?a:1))%m:1;}
int inv(int a,int m){return mbk((long long)a,m-2,m);}
long long d[300010]={1,1};
long long e[300010]={};
int M=1e9+7;
int aCb(int a,int b,int m){
  long long s=1,i;
  if(b<0)return 0;
  if(b>a-b)b=a-b;
  return (((d[a]*e[b])%M)*e[a-b])%M;
  for(i=0;i<b;i++)s=(((s*(a-i))%m)*inv(i+1,m))%m;  
  return (int)s;
}
int main(){
  int h,w,k,a,i;
  long long s,b;
  for(i=2;i<300005;i++)d[i]=(d[i-1]*i)%M;
  for(i=0;i<300005;i++)e[i]=inv(d[i],M);
  scanf("%d %d %d",&h,&w,&k);
  for(i=0;i<=k;i++){
    a=k-i;
    b=aCb(h+w+k+k,h+i+i,M);
    b=(b*(aCb(h+i+i,i,M)-aCb(h+i+i,i-1,M)+M)%M)%M;
    b=(b*(aCb(w+a+a,a,M)-aCb(w+a+a,a-1,M)+M)%M)%M;
    s=(s+b)%M;
  }
  printf("%lld\n",s);
  return 0;
}
    