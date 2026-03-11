#include<stdio.h>

int main(){
  int w,h,n,x,y,a,g[100][100]={},cnt=0;
  scanf("%d %d %d",&w,&h,&n);
  for(int i=0;i<n;i++){
    scanf("%d %d %d",&x,&y,&a);
    if(a==1){
      for(int j=0;j<x;j++)
        for(int k=0;k<h;k++)
          g[j][k]=1;
    }
    if(a==2){
      for(int j=x;j<w;j++)
        for(int k=0;k<h;k++)
          g[j][k]=1;
    }
    if(a==3){
      for(int j=0;j<w;j++)
        for(int k=0;k<y;k++)
          g[j][k]=1;
    }
    if(a==4){
      for(int j=0;j<w;j++)
        for(int k=y;k<h;k++)
          g[j][k]=1;
    }
  }
  for(int i=0;i<w;i++)
    for(int j=0;j<h;j++)
      if(g[i][j]==0)
        cnt++;
  printf("%d\n",cnt);
}