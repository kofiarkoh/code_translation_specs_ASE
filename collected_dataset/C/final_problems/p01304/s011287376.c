#include<stdio.h>
int main(){
  int n,m,i,j,k,x[110],y[110],s[110],t[110],a,b,h,w;
  scanf("%d",&m);
  while(m--){
    int d[20][20]={0};
    scanf("%d %d %d",&w,&h,&n);
    for(i=0;i<n;i++)scanf("%d %d %d %d",&x[i],&y[i],&s[i],&t[i]);
    //for(i=0;i<=h;i++)d[i][0]=1;
    //for(i=0;i<=w;i++)d[0][i]=1;
    d[0][0]=1;
    for(i=0;i<=h;i++){
      for(j=0;j<=w;j++){
	a=b=1;
	for(k=0;k<n;k++){
	  if(y[k]+t[k]==i+i+1&&x[k]+s[k]==j+j  )a=0;
	  if(y[k]+t[k]==i+i  &&x[k]+s[k]==j+j+1)b=0;
	  }//*/
	d[i+1][j]+=d[i][j]*a;
	d[i][j+1]+=d[i][j]*b;
      }
    }
    d[h][w]?printf("%d\n",d[h][w]):printf("Miserable Hokusai!\n");
  }
  return 0;
}