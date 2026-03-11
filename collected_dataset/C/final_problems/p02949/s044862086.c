#include<stdio.h>
int main(){
  int n,m,p,i,j;
  long long a[5010],b[5010],c[5010],d[3000],e[3000]={};
  scanf("%d %d %d\n",&n,&m,&p);
  for(i=0;i<m;i++){
    scanf("%lld %lld %lld",&a[i],&b[i],&c[i]);
    a[i]--;
    b[i]--;
    c[i]=p-c[i];
  }
  for(i=0;i<n;i++)d[i]=1e9;
  d[0]=0;
  e[n-1]=1;
  for(i=0;i<n;i++){
    for(j=0;j<m;j++){
      if(d[a[j]]+c[j]<d[b[j]]){
	d[b[j]]=d[a[j]]+c[j];
	if(e[b[j]])e[a[j]]=1;
	if(i+1==n&&e[b[j]]){
	  printf("-1\n");
	  return 0;
	}
      }
    }
  }
  if(d[n-1]>0)d[n-1]=0;
  printf("%lld\n",-d[n-1]);
  return 0;
}
