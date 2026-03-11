#include<stdio.h>
char d[1000010];
int w,h,dd[1000010],p[1000010]={0};
void u(int i,int j){
  int a,b,k,x,y;
  int X[]={0,1,0,-1};
  int Y[]={1,0,-1,0};
  for(k=0;k<4;k++){
    y=i+Y[k];
    x=j+X[k];
    if(y<0||x<0||w==x||h==y||d[y*1000+x]=='#')continue;
    for(a=i*1000+j+1;p[a];a=p[a]);
    for(b=y*1000+x+1;p[b];b=p[b]);
    if(a-b){
      if(dd[a]< dd[b]) p[a]=b;
      if(dd[a]>=dd[b]) p[b]=a;
      if(dd[a]==dd[b])dd[b]++;
    }
  }
}
int main(){
  int a,b,c,n,i,j,k,mi,mj;
  scanf("%d %d",&w,&h);
  for(i=0;i<h;i++){
    for(j=0;j<w;j++){
      scanf(" %c",&d[i*1000+j]);
      if(d[i*1000+j]=='t')d[mi=i*1000+j]='.';
    }
  }
  for(i=0;i<h;i++){
    for(j=0;j<w;j++){
      if(d[i*1000+j]=='#')continue;
      u(i,j);
    }
  }
  scanf("%d",&n);
  for(c=0;c<=n;c++){
    for(a=1;p[a];a=p[a]);
    for(mj=mi+1;p[mj];mj=p[mj]);
    if(mj==a){
      printf("%d\n",c);
      break;
    }
    if(c==n){
      printf("-1\n");
      break;
    }
    scanf("%d %d",&j,&i);
    if(d[i*1000+j]=='.')continue;
    d[i*1000+j]='.';
    u(i,j);
  }
  while(c++<n)scanf("%*d %*d");
  return 0;
}
   