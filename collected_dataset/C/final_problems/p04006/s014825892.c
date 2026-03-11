#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,a[2010],b[2010],min=1e9,s,i,j;
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++)scanf("%d",&a[i]);
  for(i=0;i<n;i++)b[i]=1e9;
  for(i=0;i<n;i++){
    for(j=0;j<n;j++)b[j]=MIN(b[j],a[(j-i+n)%n]);
    for(j=s=0;j<n;j++)s+=b[j];
    min=MIN(min,s+m*i);
  }
  printf("%d\n",min);
  return 0;
}
