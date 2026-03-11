#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
int MAX3(int a,int b,int c){return MAX(a,MAX(b,c));}
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,d,v,s,i,j,min=1e9,b[1010]={};
  int a[2000]={};
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d %d %d %d",&m,&d,&v,&s);
    for(j=0;j<v;j++)a[j+(m-1)*30+(d-1)+360]=MAX(a[j+(m-1)*30+(d-1)+360],s);
  }
  for(i=0;i<1200;i++)a[i+1]=MAX(a[i+1],a[i]-1);
  for(i=1200;i;i--)  a[i-1]=MAX(a[i-1],a[i]-1);
  for(i=0;i<360;i++)min=MIN(min,MAX3(a[i],a[i+360],a[i+720]));
  printf("%d\n",min);
  return 0;
}


  