#include<stdio.h>
int Y[]={0,1,0,-1};
int X[]={1,0,-1,0};
int main(){
  int h,w,x[30][3],y[30][3],i,j,k,l,ny,nx,p=0,r[30]={0};
  char s[510][510]={0};
  scanf("%d %d",&h,&w);
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      scanf(" %c",&s[i][j]);
      if(s[i][j]-'.'){
	p=s[i][j]-'A';
	y[p][r[p]  ]=i;
	x[p][r[p]++]=j;
      }
    }
  }
  p=0;
  FRONT:;
  for(i=0;i<26;i++){
    if(r[i]==0)continue;
    int d[30][30]={0};
    for(l=0;l<2;l++){
      for(j=0;j<4;j++){
	for(k=1;1;k++){
	  ny=Y[j]*k+y[i][l];
	  nx=X[j]*k+x[i][l];
	  if(s[ny][nx]-'.')break;
	  d[ny][nx]++;
	  if(d[ny][nx]==2){
	    s[y[i][0]][x[i][0]]=s[y[i][1]][x[i][1]]='.';
	    p+=2;
	    r[i]=0;
	    goto FRONT;
	  }
	}
      }
    }
  }
  printf("%d\n",p);
  return 0;
}