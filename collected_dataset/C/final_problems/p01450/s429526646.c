#include<stdio.h>
int n,m,a,M=1e9+7,d[210][20010]={1},i,j,ans=0,w[210],s[210]={};
int main(){
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    for(j=i;j&&w[j-1]<a;j--)w[j]=w[j-1];
    w[j]=a;
  }
  //for(i=0;i<n;i++)printf("%d ",w[i]);printf("\n");
  for(i=n;i;i--)s[i-1]=s[i]+w[i-1];
  //for(i=0;i<=n;i++)printf("%d ",s[i]);printf("\n");
  for(i=0;i<n;i++){
    for(j=0;j<m;j++){
      d[i+1][j+w[i]]=(d[i+1][j+w[i]]+d[i][j])%M;
      d[i+1][j         ]=(d[i+1][j     ]+d[i][j])%m;
    }
  }
  /*for(i=0;i<=n;i++){
    for(j=0;j<=m;j++)printf("%d ",d[i][j]);printf("\n");
    }//*/
  for(i=0;i<n;i++){
    for(j=0;j<w[i];j++){
      if(m-j-1-s[i+1]<0)continue;
      ans=(ans+d[i][m-1-j-s[i+1]])%M;
    }
  }
  printf("%d\n",ans);
  return 0;
}
 

