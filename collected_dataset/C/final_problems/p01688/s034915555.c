#include<stdio.h>
int main(){
  int n,m,i,j,s[100],d[100]={};
  int a,dr=0,er=0,e[100]={};
  char c,w;
  scanf("%d %d",&n,&m);
  while(m--){
    scanf(" %c %d",&c,&a);
    if(c=='D'){
      for(j=dr++;j&&d[j-1]<a;j--)d[j]=d[j-1];
      d[j]=a;
    }
  }
  scanf("%d",&m);
  while(m--){
    scanf(" %c%c %d",&c,&w,&a);
    if(c==w&&c=='D'){
      for(j=er++;j&&e[j-1]<a;j--)e[j]=e[j-1];
      e[j]=a;
    }
  }
  for(i=j=a=0;n>1;){//printf("%d %d %d %d %d\n",d[i],d[i+1],e[j],n,a);
    if(d[i]+d[i+1]>e[j]){
      a+=d[i++];
      n--;
    }
    else{
      a+=e[j++];
      n-=2;
    }
  }
  if(n)a+=d[i];
  printf("%d\n",a);
  return 0;
}
      