#include<stdio.h>
int n,w[10],h[10],a[10];
long long min;
void f(int d,int t[10],int c[10],int y[10]){
  int i,j;
  if(!d){
    long long g=0;
    for(i=0;i<n;i++)g+=w[t[i]]*i;
    if(min>g){
      min=g;
      for(i=0;i<n;i++)a[i]=t[i];
    }
    return;
  }
  for(i=0;i<n;i++){
    if(c[i])continue;//printf("%d %d\n",d,i);
    t[n-d]=i;
    c[i]=1;
    y[n-d]=h[i];
    for(j=0;j<n-d;j++){
      y[j]-=w[i];
      if(y[j]<0)break;
    }
    if((n-d)-j==0)f(d-1,t,c,y);
    c[i]=0;
    for(j=0;j<n-d;j++)y[j]+=w[i];      
  }
  return;
}      
int main(){
  char s[10][12];
  int i;
  while(scanf("%d",&n),n){
    min=2100000000;
    int t[10]={0};
    int c[10]={0};
    int y[10]={0};
    for(i=0;i<n;i++){
      scanf("%s %d %d",s[i],&w[i],&h[i]);
      a[i]=0;
    }
    f(n,t,c,y);
    for(i=0;i<n;i++)printf("%s\n",s[a[i]]);
  }
  return 0;
}