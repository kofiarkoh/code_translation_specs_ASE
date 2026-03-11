#include<stdio.h>

int use[11];
int count;
int dis[11][11];
int dp[20][20][11];
int x[11],y[11];

int mindis(int start){
  int i;
  int tmp,min;
  int flag;
  min = 0;
  flag =0;
  for(i=1;i<=count;i++){
    if(!use[i]){
      use[i] = 1;
      tmp = mindis(i) + dis[start][i];
      use[i] = 0;
      if(!min)min = tmp;
      else if(min > tmp)min = tmp;
      flag = 1;
    }
  }
  if(flag)return min;
  else return 0;
}

int main(void){
  char map[20][20];
  int i,j,k;
  int w,h;
  int min;
  int flag1,flag2;

  while(1){
    scanf("%d %d",&w,&h);
    if(!w && !h)break;

    count = 0;
    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	scanf(" %c ",&map[i][j]);
	if(map[i][j]=='o'){
	  x[0] = j;
	  y[0] = i;
	}
	else if(map[i][j]=='*'){
	  x[count+1] = j;
	  y[count+1] = i;
	  count++;
	} 
      }
    }

    for(i=0;i<h;i++){
      for(j=0;j<w;j++){
	for(k=0;k<=count;k++){
	  dp[i][j][k] = -1;
	}
      }
    }

    flag1 = 1;
    for(k=0;k<=count;k++){
      dp[y[k]][x[k]][k] = 0;

      flag2 = 1;
      while(flag2){

	flag2 = 0;
	for(i=0;i<h;i++){
	  for(j=0;j<w;j++){
	    if(dp[i][j][k]>=0){
	      
	      if(i+1<h && map[i+1][j]!='x'){
		if(dp[i+1][j][k]<0 || dp[i+1][j][k]>dp[i][j][k]+1){
		  dp[i+1][j][k] = dp[i][j][k] + 1;
		  flag2 = 1;
		}
	      }
	      if(j+1<w && map[i][j+1]!='x'){
		if(dp[i][j+1][k]<0 || dp[i][j+1][k]>dp[i][j][k]+1){
		  dp[i][j+1][k] = dp[i][j][k] + 1;
		  flag2 = 1;
		}
	      }
	      if(i-1>=0 && map[i-1][j]!='x'){
		if(dp[i-1][j][k]<0 || dp[i-1][j][k]>dp[i][j][k]+1){
		  dp[i-1][j][k] = dp[i][j][k] + 1;
		  flag2 = 1;
		}
	      }
	      if(j-1>=0 && map[i][j-1]!='x'){
		if(dp[i][j-1][k]<0 || dp[i][j-1][k]>dp[i][j][k]+1){
		  dp[i][j-1][k] = dp[i][j][k] + 1;
		  flag2 = 1;
		}
	      }
	    }
	  }
	}
      }
      if(!k){
	for(i=1;i<=count;i++){
	  if(dp[y[i]][x[i]][k] < 0)flag1 = 0;
	}
      }
    
      if(!flag1)break;

      for(i=0;i<=count;i++){
	if(i!=k)dis[k][i] = dp[y[i]][x[i]][k];
	else dis[k][i] = 0;
      }

    }

    if(!flag1){
      printf("-1\n");
    } else {
      for(i=1;i<=count;i++)use[i] = 0;
      use[0] = 1;
      min = mindis(0);
      printf("%d\n",min);
    }
  }
  return 0;
}