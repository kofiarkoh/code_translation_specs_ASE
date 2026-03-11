#include<stdio.h>
int main(){
  int w=-1,h,f[110][110],d[110][110],i,j,k,p,b,c,m=1;
  char s[15];
  while(scanf("%d",&h),h){
    int t[110]={0};
    int y[110]={0};
    if(w+1)printf("\n");
    scanf("%d",&w);
    for(i=c=0;i<=h;i++){
      for(j=0;j<=w;j++){
	scanf("%s",s);
	d[i][j]=f[i][j]=0;
	if(s[0]=='?'){
	  y[i]++;
	  t[j]++;
	  f[i][j]=1;
	  c++;
	}
	else{
	  m=1;
	  if(s[k=0]=='-'){
	    m=-1;
	    k++;
	  }
	  for(;s[k];k++)d[i][j]=d[i][j]*10+s[k]-'0';
	  d[i][j]*=m;
	}
      }
    }
    while(c){
      b=1;
      for(j=0;j<w;j++){
	if(t[j]-1)continue;
	for(i=b=t[j]=0;i<h;i++){
	  if(f[i][j]==1)p=i;
	  d[h][j]-=d[i][j];
	}
	d[p][j]=d[h][j];
	y[p]--;
	f[p][j]=2;
	c--;
      }
      for(i=0;i<h;i++){
	if(y[i]-1)continue;
	for(j=b=y[i]=0;j<w;j++){
	  if(f[i][j]==1)p=j;
	  d[i][w]-=d[i][j];
	}
	d[i][p]=d[i][w];
	t[p]--;
	f[i][p]=2;
	c--;
      }
      if(b)break;
    }
    if(c)printf("NO\n");
    else{
      for(i=0;i<h;i++){
	for(j=0;j<w;j++){
	  if(f[i][j])printf("%d\n",d[i][j]);
	}//printf("\n");
      }
    }
  }
  return 0;
}