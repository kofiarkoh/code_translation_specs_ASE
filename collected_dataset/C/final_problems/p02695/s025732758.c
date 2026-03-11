#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include<math.h>
#define ll long long
int main(){
  ll int n,m,q,ans=0,max=0;
  scanf("%lld%lld%lld",&n,&m,&q);
  ll int a[q],b[q],c[q],d[q],A[11];
  for(int i=1;i<=q;i++){
    scanf("%lld%lld%lld%lld",&a[i],&b[i],&c[i],&d[i]);
  }
  for(int i=1;i<=m;i++){
    A[1]=i;
    for(int j=i;j<=m;j++){
      A[2]=j;
      for(int k=j;k<=m;k++){
        A[3]=k;
        for(int l=k;l<=m;l++){
          A[4]=l;
          for(int z=l;z<=m;z++){
            A[5]=z;
            for(int y=z;y<=m;y++){
              A[6]=y;
              for(int o=y;o<=m;o++){
                A[7]=o;
                for(int p=o;p<=m;p++){
                  A[8]=p;
                  for(int r=p;r<=m;r++){
                    A[9]=r;
                    for(int s=r;s<=m;s++){
                      A[10]=s;
  for(int u=1;u<=q;u++){
    if(A[b[u]]-A[a[u]]==c[u]){
      ans+=d[u];
    }
  }
                      if(max<ans){
                        max=ans;
                      }
                      ans=0;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  printf("%lld",max);
  return 0;
}