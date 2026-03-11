#include<stdio.h>
int main(){
  int X[]={0,1,0,-1};
  int Y[]={1,0,-1,0};	   
  int w,h,i,j,ny,nx,k,l,x[3000],y[3000],t,r;
  char s[55][55];
  while(scanf("%d %d",&w,&h),w||h){
    int c[4]={0};
    int e[55][55]={0};
    for(i=0;i<h;i++)scanf("%s",s[i+1]+1);
    for(i=1;i<=h;i++){
      for(j=1;j<=w;j++){
	if(s[i][j]=='.')continue;
	if(s[i][j]=='W')k=2;
	else            k=1;
	y[0  ]=i;
	x[t=0]=j;
	for(r=1;r-t;t++){//printf("%d\n",r);
	  for(l=0;l<4;l++){
	    ny=y[t]+Y[l];
	    nx=x[t]+X[l];
	    if(s[ny][nx]-'.'||e[ny][nx]==k||e[ny][nx]==3)continue;
	    e[y[r]=ny][x[r]=nx]+=k;
	    r++;
	  }
	}
      }
    }
    for(i=0;i<h;i++){
      for(j=0;j<w;j++)c[e[i+1][j+1]]++;
    }
    printf("%d %d\n",c[1],c[2]);
  }
  return 0;
}