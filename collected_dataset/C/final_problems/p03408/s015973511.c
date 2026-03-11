#include<stdio.h>
#include<string.h>
int main(){
  char p[101][11];
  int i,j,n,m,x[101]={},t=0;
  scanf("%d",&n);
  for(i=0;i<n;i++){scanf("%s",p[t]);for(j=0;j<t;j++)if(!strcmp(p[t],p[j])){x[j]++;j=t;}if(j==t){x[t]++;t++;}}
  scanf("%d",&m);
  for(i=0;i<m;i++){scanf("%s",p[100]);for(j=0;j<t;j++)if(!strcmp(p[100],p[j])){x[j]--;}}
  j=0;for(i=0;i<t;i++)j=x[i]<j?j:x[i];
  printf("%d",j);
}
