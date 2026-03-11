#include<stdio.h>

int main(void){
  char map[20][20];
  int dp[20][20];
  int i,j,k,l;
  int w,h;
  int w0,h0;
  int count;
  int min,minw,minh;
  int sum;

  while(1){
    scanf("%d %d",&w,&h);
    if(!w && !h)break;

    count = 0;
    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	dp[i][j] = -1;
	scanf(" %c ",&map[i][j]);
	if(map[i][j]=='o'){
	  w0 = j;
	  h0 = i;
	  dp[i][j] = 0;
	}
	else if(map[i][j]=='*'){
	  count++;
	} 
      }
    }

    sum = 0;
    for(l=0;l<count;l++){
      min = 210;
      for(k=0;k<210;k++){

	for(i=0;i<h;i++){
	  for(j=0;j<w;j++){
	    if(dp[i][j]>=0){
	      
	      if(i+1<h && map[i+1][j]!='x'){
		if(dp[i+1][j]<0 || dp[i+1][j]>dp[i][j]+1){
		  dp[i+1][j] = dp[i][j] + 1;
		}
	      }
	      if(i-1>=0 && map[i-1][j]!='x'){
		if(dp[i-1][j]<0 || dp[i-1][j]>dp[i][j]+1){
		  dp[i-1][j] = dp[i][j] + 1;
		}
	      }	
	      if(j+1<w && map[i][j+1]!='x'){
		if(dp[i][j+1]<0 || dp[i][j+1]>dp[i][j]+1){
		  dp[i][j+1] = dp[i][j] + 1;
		}
	      }
	      if(j-1>=0 && map[i][j-1]!='x'){
		if(dp[i][j-1]<0 || dp[i][j-1]>dp[i][j]+1){
		  dp[i][j-1] = dp[i][j] + 1;
		}
	      }

	    }
	  }
	}

      }

      for(i=0;i<h;i++){
	for(j=0;j<w;j++){
	  if(map[i][j]=='*'){
	    if(dp[i][j]<min){
	      min = dp[i][j];
	      minw = j;
	      minh = i;
	    }
	  }
	}
      }

      if(min==210)break;
      else{
	sum += min;
	map[h0][w0] = '.';
	w0 = minw;
	h0 = minh;
	map[h0][w0] = 'o';
	for(i=0;i<h;i++)for(j=0;j<w;j++)dp[i][j] = -1;
	dp[h0][w0] = 0;
      }

    }
    if(min == 210)printf("-1\n");
    else printf("%d\n",sum);
  }
  return 0;
}