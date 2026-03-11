#include<stdio.h>
int Y[]={0,1,0,-1};
int X[]={1,0,-1,0};
int main(){
  int i,j,k,l,h,w,ans=0,r[30]={0};
  int y[30][2],x[30][2],t[30]={0};
  char d[510][510]={0};
  scanf("%d %d",&h,&w);
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      scanf(" %c",&d[i][j]);
      if(d[i][j]-'.'){
	k=d[i][j]-'A';
	y[k][r[k]  ]=i;
	x[k][r[k]++]=j;
	t[k]=1;
      }
    }
  }
  for(i=0;i<26;i++){
  FRONT:
    if(t[i]==0)continue;
    int f[510][510]={0};
    for(j=0;j<2;j++){
      for(k=0;k<4;k++){
	for(l=1;d[y[i][j]+Y[k]*l][x[i][j]+X[k]*l]=='.';l++){
	  f[y[i][j]+Y[k]*l][x[i][j]+X[k]*l]++;
	  if(f[y[i][j]+Y[k]*l][x[i][j]+X[k]*l]==2){
	    ans+=2;
	    d[y[i][0]][x[i][0]]=d[y[i][1]][x[i][1]]='.';
	    i=t[i]=0;
	    goto FRONT;
	  }
	}
      }
    }
  }
  printf("%d\n",ans);
  return 0;
}