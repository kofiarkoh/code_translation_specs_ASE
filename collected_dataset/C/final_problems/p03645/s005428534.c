#include<stdio.h>

int main(){


  int n,m,a[201000],b[201000];

  scanf("%d%d",&n,&m);
  int i,j,res=0,st;
  for(i=0;i<m;i++){
    scanf("%d%d",&a[i],&b[i]);
  }

  for(i=0;i<m;i++){
    if(res==1)break;
    if(a[i]==1){
      st=b[i];
      for(j=0;j<m;j++){
        if(a[j]==st && b[j]==n){
          res=1;
          break;
        }
      }
    }
  }


  if(res==1)printf("POSSIBLE\n");
  else printf("IMPOSSIBLE\n");

  return 0;

}




