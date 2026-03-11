#include<stdio.h>
int main(){
  int h,w,i,j,y[30010],x[30010],c[30010],r,t,ny,nx,a,b,n;
  int Y[]={-1,-1,0,0,1,1};
  int X[]={-1,1,-2,2,-1,1};
  while(scanf("%d %d",&w,&h),w){
    int max=0,d[200][300]={};
    for(i=1;i<=h;i++){
      for(j=1;j<=w;j++)d[i][j*2+1-i%2]=1e9;
    }
    scanf("%d",&n);
    for(i=r=0;i<n;i++){
      scanf("%d %d",&a,&b);
      a=a*2+1-b%2;
      d[y[r]=b][x[r]=a]=1;
      r++;
    }
    for(t=0;r-t;t++){
      for(i=0;i<6;i++){
	ny=y[t]+Y[i];
	nx=x[t]+X[i];
	if(d[y[t]][x[t]]+1>=d[ny][nx])continue;
	d[y[r]=ny][x[r]=nx]=d[y[t]][x[t]]+1;
	r++;
      }
    }
    /*for(i=0;i<h+2;i++){
      for(j=0;j<w*2+3;j++)printf("%d ",d[i][j]);
      printf("\n");
      }//*/
    scanf("%d",&n);
    while(n--){
      int f[200][300]={};
      scanf("%d %d",&a,&b);
      y[0]=b;
      x[0]=a*2+1-b%2;
      c[t=0]=1;
      if(d[b][a]==1)continue;
      for(r=1;r-t;t++){
	for(i=0;i<6;i++){
	  ny=y[t]+Y[i];
	  nx=x[t]+X[i];
	  if(d[ny][nx]<=c[t]+1||f[ny][nx])continue;
	  f[y[r]=ny][x[r]=nx]=1;;
	  c[r++]=c[t]+1;
	}
      }
      if(max<r)max=r;
    }
    printf("%d\n",max);
  }
  return 0;
}
    