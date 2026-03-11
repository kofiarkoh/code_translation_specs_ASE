#include<stdio.h>
char p[1000020]={1,1};
int main(){
  long long n,i,j,s,d[100010],c,M=1000010;
  for(i=0;i<M;i++){
    if(p[i])continue;
    for(j=i*i;j<M;j+=i)p[j]=1;
  }
  for(i=c=0;i<M;i++){
    if(p[i]==0)d[c++]=i;
  }
  while(scanf("%lld",&n),n){
    s=1;
    for(i=0;n&&i<c;i++){
      for(j=0;n%d[i]==0;j++)n/=d[i];//printf("%lld %lld\n",d[i],j);
      s*=j+j+1;
    }
    printf("%lld\n",s/2+1);
  }
  return 0;
}