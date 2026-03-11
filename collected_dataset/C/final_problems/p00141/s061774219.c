#include <stdio.h>

#define WL '#' /* WALL  */
#define BL ' ' /* BLANK */

int main(){
  char g[100][101];
  int d,i,j,k,n,px,py,v,w;
  int dx[]={-1,0,1,0},dy[]={0,-1,0,1};

  scanf("%d\n",&d);
  for(i=0;i<d;i++){
    if(i!=0) printf("\n");
    scanf("%d\n",&n);
    for(j=0;j<n;j++){
      for(k=1;k<n-1;k++) g[j][k]=BL;
      g[j][0]=g[0][j]=g[j][n-1]=WL;
      g[j][n]=0;
    }
    px=py=n-1;
    v=0;w=0;
    while(1){
      if(g[py+dy[v]][px+dx[v]]==BL){
        g[py][px]=WL; px+=dx[v]; py+=dy[v]; w++;
      } else {
        if(g[py][px]==WL || w==2){
          break;
        } else {
          px-=dx[v]; py-=dy[v]; v=(v==3?0:v+1); w=0;
        }
      }
    }
    if(n==2) g[1][1]=BL;
    for(j=0;j<n;j++) printf("%s\n",g[j]);
  }
  return 0;
}