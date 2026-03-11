#include<stdio.h>
long long MAX(long long a,long long b){return a<b?b:a;}
long long n,m,i,j,k,a[3][3],d[2][5010*5010]={};
int main(){
  scanf("%lld",&n);
  for(i=0;i<2;i++){
    for(j=0;j<3;j++)scanf("%lld",&a[i][j]);
  }
  for(j=0;j<3;j++)a[2][j]=a[0][j];
  for(k=0;k<2;k++){
    for(i=n;i;i--){
      for(j=0;j<3;j++){
	if(a[k][j]>i)continue;
	d[k][i-a[k][j]]=MAX(d[k][i-a[k][j]],d[k][i]+a[k+1][j]);
      }
    }
    for(i=m=0;i<n;i++)m=MAX(m,d[k][i]+i);
    n=m;
  }
  printf("%lld\n",n);
  return 0;
}
