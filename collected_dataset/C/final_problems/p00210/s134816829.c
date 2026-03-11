#include<stdio.h>
int main(){
  int X[]={1,0,-1,0};
  int Y[]={0,-1,0,1};
  int h,w,i,j,rr,r,t,dr[1000],y[1000],x[1000],gy[1000],gx[1000],f,ny,nx;
  char d[40][40]={0};
  while(scanf("%d %d",&w,&h),w||h){
    for(i=r=rr=0;i<h;i++){
      for(j=0;j<w;j++){
	scanf(" %c",&d[i][j]);
	if(d[i][j]=='E')dr[r++]=0;
	if(d[i][j]=='N')dr[r++]=1;
	if(d[i][j]=='W')dr[r++]=2;
	if(d[i][j]=='S')dr[r++]=3;
	if(d[i][j]-'#'&&d[i][j]-'.'&&d[i][j]-'X')d[y[r-1]=i][x[r-1]=j]='#';
	if(d[i][j]=='X'){
	  d[gy[rr]=i][gx[rr]=j]='.';
	  rr++;
	}
      }//printf("%s\n",d[i]);
    }
    for(t=0;t<181;t++){
      for(i=f=0;i<r;i++){//printf("%d:%d %d %d\n",i,y[i],x[i],dr[i]);
	if(y[i]==0)continue;
	f=1;
	for(j=3;j<7;j++){
	  ny=y[i]+Y[(j+dr[i])%4];
	  nx=x[i]+X[(j+dr[i])%4];
	  if(d[ny][nx]=='#')continue;
	  dr[i]=(j+dr[i])%4;//printf("%d:%d %d %d\n\n",i,y[i],x[i],dr[i]);
	  break;
	}
      }
      if(f==0)break;
      for(j=2;j<6;j++){
	for(i=0;i<r;i++){
	  if(dr[i]-(j%4)||y[i]==0)continue;
	  ny=y[i]+Y[j%4];
	  nx=x[i]+X[j%4];
	  if(d[ny][nx]=='.'){
	    d[y[i]   ][x[i]   ]='.';
	    d[y[i]=ny][x[i]=nx]='#';
	  }
	}
      }
      for(i=0;i<r;i++){
	if(y[i]==0)continue;
	for(j=0;j<rr;j++){
	  if(y[i]==gy[j]&&x[i]==gx[j]){
	    y[i]=0;
	    d[gy[j]][gx[j]]='.';
	  }
	}
      }
    }
    if(t-181)printf("%d\n",t);
    else     printf("NA\n");
  }
  return 0;
}