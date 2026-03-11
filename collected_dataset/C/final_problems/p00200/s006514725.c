#include<stdio.h>
int main(){
  int i,j,k,l,n,m;
  int d[2][100][100]={0};
  while(scanf("%d %d",&m,&n),n||m){
    for(i=0;i<100;i++){
      for(j=0;j<100;j++)d[0][i][j]=d[1][i][j]=100000000;
      d[0][i][i]=d[1][i][i]=0;
    }
    while(m--){
      scanf("%d %d %d %d",&i,&j,&k,&l);
      d[0][i-1][j-1]=d[0][j-1][i-1]=k;
      d[1][i-1][j-1]=d[1][j-1][i-1]=l;
    }
    for(l=0;l<n;l++){
      //for(i=0;i<2;i++){
      for(j=0;j<n;j++){
	for(k=0;k<n;k++){
	  //for(l=0;l<n;l++){
	  for(i=0;i<2;i++){
	    if(d[i][j][k]>d[i][j][l]+d[i][l][k]){
	       d[i][j][k]=d[i][j][l]+d[i][l][k];
	    }
	  }
	}
      }
    }
    scanf("%d",&l);
    while(l--){
      scanf("%d %d %d",&i,&j,&k);
      printf("%d\n",d[k][i-1][j-1]);
    }
  }
  return 0;
}
      