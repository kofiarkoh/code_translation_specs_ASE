#include<stdio.h>
int Y[]={0,1,0,-1};
int X[]={1,0,-1,0};
int MIN(int a,int b){return a<b?a:b;}
int MAX(int a,int b){return a<b?b:a;}
int main(){
  int h,w,n,m,i,j,x[10010],y[10010],ny,nx,f[100][100]={};
  int a,b,c,t,r,l[110],u[100][100]={};
  char d[100][100];
  scanf("%d %d %d %d",&h,&w,&n,&m);
  for(i=0;i<h;i++){
    for(j=0;j<w;j++){
      scanf(" %c",&d[i][j]);
      if(d[i][j]=='D')d[y[0]=i][x[0]=j]='.';
    }
  }
  f[y[t=0]][x[0]]=1;
  for(r=1;r-t;t++){
    for(i=0;i<4;i++){
      ny=y[t]+Y[i];
      nx=x[t]+X[i];
      if(ny<0||h<=ny||nx<0||w<=nx)continue;
      if(d[ny][nx]=='#'||f[ny][nx])continue;
      f[y[r]=ny][x[r]=nx]=1;
      r++;
    }
  }
  for(i=0;i<n;i++)scanf("%d",&l[i]);
  l[n]=100;
  for(i=0;i<m;i++){
    scanf("%d %d %d",&a,&b,&c);
    u[MAX(0,b-l[c]  )][MAX(0,a-l[c]  )]++;
    u[MAX(0,b-l[c]  )][MIN(w,a+l[c]+1)]--;
    u[MIN(h,b+l[c]+1)][MAX(0,a-l[c]  )]--;
    u[MIN(h,b+l[c]+1)][MIN(w,a+l[c]+1)]++;
    if(c--){
      u[MAX(0,b-l[c]  )][MAX(0  ,a-l[c]  )]--;
      u[MAX(0,b-l[c]  )][MIN(w,a+l[c]+1)]++;
      u[MIN(h,b+l[c]+1)][MAX(0,a-l[c]  )]++;
      u[MIN(h,b+l[c]+1)][MIN(w,a+l[c]+1)]--;
    }
  }
  for(i=0;i<h;i++){
    for(j=1;j<w;j++)u[i][j]+=u[i][j-1];
  }
  for(j=a=b=0;j<w;j++){
    for(i=0;i<h;i++){
      if(i)u[i][j]+=u[i-1][j];
      if(u[i][j]==m&&d[i][j]=='.'){//printf("%d ",u[i][j]);
	a++;
	b+=f[i][j];
      }
    }
  }
  if(a==0)printf("Broken\n");
  else if(b%a)printf("Unknown\n");
  else printf("%s\n",b?"Yes":"No");
  return 0;
}
  
      
      
    
       