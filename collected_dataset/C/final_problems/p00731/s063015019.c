#include<stdio.h>
int main(){
  int w,h,i,j,k,x,y,nx,ny,min,m,han,a;
  char g;
  int Y[9]={2,1,1,0,0,0,-1,-1,-2};
  int X[9]={1,1,2,1,2,3,1,2,1};
  int c[2][62][32],f[2][62][32],d[62][32];
  while(scanf("%d %d",&w,&h),w||h){
    int ans=2100000000;
    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	c[0][i][j]=c[1][i][j]=200000000;
	f[0][i][j]=f[1][i][j]=0;
	scanf(" %c",&g);
	if     (g=='S')d[i][j]=c[0][i][j]=c[1][i][j]=0;
	else if(g=='T')d[i][j]=-1;
	else if(g=='X')d[i][j]=100000000;
	else           d[i][j]=g-'0';
	if(g=='X'||g=='T')f[0][i][j]=f[1][i][j]=2;
      }
    }
    while(han=1){
      min=2100000000;
      for(i=0;i<h;i++){
	for(j=0;j<w;j++){
	  for(k=0;k<2;k++){
	    if(f[k][i][j])continue;
	    han=0;
	    if(min>c[k][i][j]){
	      y=i;
	      x=j;
	      m=a=1-k;
	      min=c[k][i][j];
	    }
	  }
	}
      }
      if(han)break;//printf("%d %d\n",y,x);
      if(m==0)m--;
      for(i=0;i<9;i++){
	ny=y+Y[i];
	nx=x+X[i]*m;
	if(nx<0||w<=nx||ny<0||h<=ny)continue;
	if(d[ny][nx]==-1){//printf("d%d %d\n",y,x);
	  if(ans>c[1-a][y][x])ans=c[1-a][y][x];
	  continue;
	}
	if(c[a][ny][nx]>c[1-a][y][x]+d[ny][nx]){
	   c[a][ny][nx]=c[1-a][y][x]+d[ny][nx];
	}
      }
      f[1-a][y][x]=1;
    }
    /*for(k=0;k<2;k++){
      for(i=0;i<h;i++){
	for(j=0;j<w;j++)printf("%10d",c[k][i][j]);
	printf("\n");
      }
      printf("\n");
      }//*/
    if(ans>100000000)printf("-1\n");
    else              printf("%d\n",ans);
  }
  return 0;
}