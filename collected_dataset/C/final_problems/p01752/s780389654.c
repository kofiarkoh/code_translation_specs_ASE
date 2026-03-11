#include<stdio.h>
int Y[]={0,1,0,-1};
int X[]={1,0,-1,0};
int main(){
  int h,w,y,x,d,ny,nx,nd,gy,gx,a=0,i,j,k,f[110][110][5]={};
  char m[110][110]={};
  char c[]={">v<^"};
  scanf("%d %d",&h,&w);
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      scanf(" %c",&m[i][j]);
      if(m[i][j]=='G')m[gy=i][gx=j]='.';
      for(k=0;k<4;k++){
	if(m[i][j]==c[k])break;
      }
      if(k-4){
	m[y=i][x=j]='.';
	d=k;
	f[i][j][k]=1;
      }
    }
  }
  while(gy-y||gx-x){//printf("%d %d %d\n",y,x,d);
    nd=(d+1)%4;
    ny=y+Y[d];
    nx=x+X[d];
    if(m[ny][nx]-'.'){
      ny=y;
      nx=x;
      d=(d+3)%4;
    }
    else if(m[ny+Y[nd]][nx+X[nd]]=='.')d=nd;
    if(f[ny][nx][d])break;
    if(f[ny][nx][0]+f[ny][nx][1]+f[ny][nx][2]+f[ny][nx][3]==0)a++;
    f[y=ny][x=nx][d]=1;
  }
  printf("%d\n",gy==y&&gx==x?a+1:-1);
  return 0;
}
    