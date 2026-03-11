#include<stdio.h>
int main(){
  int w,h,a,b,c,n,i,j,k,x,y,mi,mj,p[1000010]={0};
  char d[1000010];
  int X[]={0,1,0,-1};
  int Y[]={1,0,-1,0};
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
      for(k=0;k<4;k++){
	y=i+Y[k];
	x=j+X[k];
	if(y<0||x<0||w==x||h==y||d[y*1000+x]=='#')continue;
	for(a=i*1000+j+1;p[a];a=p[a]);
	for(b=y*1000+x+1;p[b];b=p[b]);
	if(a-b){
	  if(d[a]< d[b])p[a]=b;
	  if(d[a]>=d[b])p[b]=a;
	  if(d[a]==d[b])d[b]++;
	}
      }
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
    /*for(a=0;a<h;a++){
      for(b=0;b<w;b++)printf("%8d",p[a*1000+b+1]);
      printf("\n");
    }printf("\n");//*/
    for(k=0;k<4;k++){
      y=i+Y[k];
      x=j+X[k];
      if(y<0||x<0||w==x||h==y||d[y*1000+x]=='#')continue;
      for(a=i*1000+j+1;p[a];a=p[a]);
      for(b=y*1000+x+1;p[b];b=p[b]);
      if(a-b){
	if(d[a]< d[b])p[a]=b;
	if(d[a]>=d[b])p[b]=a;
	if(d[a]==d[b])d[a]++;
      }
    }
  }
  /*for(i=0;i<h;i++){
    for(j=0;j<w;j++)printf("%8d",p[i*1000+j+1]);
    printf("\n");
    }//*/
  while(c++<n)scanf("%*d %*d");
  return 0;
}
   