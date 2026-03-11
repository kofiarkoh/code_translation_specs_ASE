#include<stdio.h>
int main(){
  int n,m,i,j,max,u[510],r,s[510],k,d[10][10]={0};
  scanf("%d",&n);
  while(n--){
    for(i=0;i<5;i++){
      for(j=r=0;j<5;j++){
	scanf("%d",&m);
	d[i][j]=r=(r+1)*m;
      }
    }
    for(i=max=0;i<5;i++){
      s[u[0]=0]=d[0][i];
      for(j=r=1;j<6;j++){
	u[r]=j;
	while(s[r-1]>d[j][i]){
	  if(max<s[r-1]*(j-u[r-1]))max=s[r-1]*(j-u[r-1]);
	  r--;
	}
	if(s[r-1]-d[j][i])s[r++]=d[j][i];
      }
    }
    printf("%d\n",max);
  }
  return 0;
}