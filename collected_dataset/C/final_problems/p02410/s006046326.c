#include<stdio.h>
int main(void)
{
  int i,j,k,m,n,a[101][101],b[101],c[101];

  scanf("%d %d",&n,&m);

  for(i=1;i<=n;i++){
    for(j=1;j<=m;j++){
      scanf("%d",&a[i][j]);
    }
  }
  for(j=1;j<=m;j++){
    scanf("%d",&b[j]);
    c[j]=0;
  }

  for(i=1;i<=n;i++){
    for(j=1;j<=m;j++){
      c[i]=c[i]+a[i][j]*b[j];
    }
  }
  for(i=1;i<=n;i++){
    printf("%d\n",c[i]);
  }

  return 0;
}