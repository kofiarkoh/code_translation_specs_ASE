#include<stdio.h>
int Y[]={0,1,0,-1};
int X[]={1,0,-1,0};
int main(){
  int h,w,i,j,k,y[410*410],x[410*410],ny,nx,t,r,f[410][410]={};
  long long a,b,c=0;
  char s[410][410]={};
  scanf("%d %d",&h,&w);
  for(i=0;i<h;i++)scanf(" %s",s[i+1]+1);
  for(i=1;i<=h;i++){
    for(j=1;j<=w;j++){
      if(f[i][j])continue;
      a=b=0;
      f[y[t=0]=i][x[0]=j]=1;
      for(r=1;r-t;t++){
	if(s[y[t]][x[t]]=='.')a++;
	else                  b++;
	for(k=0;k<4;k++){
	  ny=y[t]+Y[k];
	  nx=x[t]+X[k];
	  if(f[ny][nx]==1)continue;
	  if(s[ny][nx]+s[y[t]][x[t]]-'#'-'.')continue;
	  f[y[r]=ny][x[r]=nx]=1;
	  r++;
	}
      }//printf("%d %d\n",a,b);
      c+=a*b;
    }
  }
  printf("%lld\n",c);
  return 0;
}
