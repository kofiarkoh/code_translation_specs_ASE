#include<stdio.h>
int main(){
  int n,m,i,j,l,a,b,o,h,min,mi;
  int d[21][21],f[21][2],c[21][2],s[2];
  scanf("%d %d",&n,&m);
  for(i=1;i<=n;i++){
    for(j=1;j<=n;j++)d[i][j]=100000000;
    c[i][0]=c[i][1]=100000000;
    f[i][0]=f[i][1]=0;
  }
  while(m--){
    scanf("%d,%d,%d,%d",&a,&b,&o,&h);
    d[a][b]=o;
    d[b][a]=h;
  }
  scanf("%d,%d,%d,%d",&s[0],&s[1],&h,&o);
  h-=o;
  for(l=0;l<2;l++){
    c[mi=s[l]][l]=0;
    for(i=0;i<n;i++){
      f[mi][l]=1;
      for(j=1;j<=n;j++){
	if(f[j][l])continue;
	if(c[j][l]>c[mi][l]+d[mi][j])c[j][l]=c[mi][l]+d[mi][j];
      }
      min=2100000000;
      for(j=1;j<=n;j++){//printf("%d ",c[j][l]);
	if(f[j][l])continue;
	if(min>c[j][l])min=c[mi=j][l];
      }//printf("\n");
    }//printf("\n");
    h-=c[s[1-l]][l];
  }
  printf("%d\n",h);
}