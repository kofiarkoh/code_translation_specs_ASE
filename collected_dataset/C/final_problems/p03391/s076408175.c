#include<stdio.h>
int main(void){
int n;
scanf("%d",&n);
int a[n],b[n],i,d[n],sum=0;
for(i=0;i<n;++i){
  scanf("%d %d",&a[i],&b[i]);
  d[i]=b[i]-a[i];
  if(d[i]>0) sum+=b[i];
}
printf("%d",sum);
return 0;
}