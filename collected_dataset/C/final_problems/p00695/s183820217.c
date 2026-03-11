#include<stdio.h>
int main(){
  int n,m,i,j,max,u[50],r,s[10],k,d[10][10]={0};
  scanf("%d",&n);
  while(n--){
    for(i=0;i<5;i++){
      for(j=r=0;j<5;j++){
	scanf(" %d",&m);
	if(m)d[i][j]=++r;
	else d[i][j]=r=0;//printf("%d ",d[i][j]);
      }//printf("\n");
    }
    for(j=0;j<5;j++)d[i][j]=0;
    for(i=max=0;i<5;i++){
      s[u[0]=0]=d[0][i];
      for(j=r=1;j<6;j++){
	u[r]=j;
	while(s[r-1]>d[j][i]){
	  if(max<s[r-1]*(j-u[r-1]))max=s[r-1]*(j-u[r-1]);
	  r--;
	}
	if(s[r-1]-d[j][i])s[r++]=d[j][i];
	//for(k=0;k<r;k++)printf("%d %d\n",s[k],u[k]);printf("\n");
      }//printf("\n");
    }
    printf("%d\n",max);
  }
  return 0;
}