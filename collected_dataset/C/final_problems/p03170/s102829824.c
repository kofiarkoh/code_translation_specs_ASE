#include<stdio.h>
int main(){
  int n,m,i,j,f,a[110],d[100010]={};
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++)scanf("%d",&a[i]);
  for(i=0;i<=m;i++){
    for(j=f=0;j<n;j++){
      if(i<a[j])continue;
      if(d[i-a[j]]==0)f=1;
    }
    d[i]=f;
  }
  printf("%s\n",d[m]?"First":"Second");
  return 0;
}
