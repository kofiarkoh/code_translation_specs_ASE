#include<stdio.h>

void dfs(int);

int map[21][21],visited[21],sum;

int main(void){

  int n,i,j,a,b,t;
  
  while(1){
    
    for(i=0;i<21;i++){
      for(j=0;j<21;j++){
	map[i][j]=0;
      }
      visited[i]=0;
    }
    
    sum=0;
    scanf("%d",&n);
    if(n==0)break;
    
    for(i=0;i<n-1;i++){
      scanf("%d %d %d",&a,&b,&t);
      map[a][b]=t;
    }
    dfs(1);
    
    printf("%d\n",sum/2);
  }
  
  return 0;
}

void dfs(int x){
  int i;
  
  for(i=1;i<21;i++){
    if(visited[i]==0 && map[x][i]>0){
      visited[i]=1;
      dfs(i);
    sum+=map[x][i];
    }

  }
}