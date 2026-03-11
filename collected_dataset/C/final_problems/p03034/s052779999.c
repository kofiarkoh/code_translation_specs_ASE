#include<stdio.h>
int main(){
  int n,i,j;
  long long d[100010],s,m=0;
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%lld",&d[i]);
  for(i=1;i<n;i++){
    for(j=s=0;i*j<n/2;j++){
      if((n-1)%i==0&&i*j>=n-1-i*j)break;
      s+=d[i*j]+d[n-i*j-1];
      if(m<s)m=s;
    }//printf("%d %lld\n",i,m);
  }
  printf("%lld\n",m);
  return 0;
}
