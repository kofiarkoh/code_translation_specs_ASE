#include <stdio.h>
#define N 3
#define LIMIT 30

int piace[N][N]; //パズル
int tmp[N][N]; //保存

int dx[4]={0,1,0,-1};
int dy[4]={1,0,-1,0};

int result;

int dfs(int ,int ,int ,int ,int );
int cal();


int main(){
  int i,j,k=0;
  int x,y;


  for(i = 0;i<N;i++){
    for(j=0;j<N;j++){
      scanf("%d",&piace[i][j]);
    }
  }

  for(i = 0 ;i <N ;i++){
    for(j = 0;j<N;j++){
      if(piace[i][j]==0){
	x = j;
	y = i;
      }
    }
  }

  while(1){

    for(i = 0;i<N;i++){
      for(j = 0;j<N;j++){
	tmp[i][j] = piace[i][j];
      }
    }

    result = dfs(x,y,-1,0,k);

    if(result !=-1){
      printf("%d\n",result);
      break;
    }

    for(i =0;i<N;i++){
      for(j=0;j<N;j++){
	piace[i][j] = tmp[i][j];
      }
    }

    result = -1;

    k++;

    }

    
  
     
  return 0;
}

 
int dfs(int x,int y,int p,int depth,int l){ //depth first search
  int i;
  int sum;
  int x_2,y_2;
  int res;


  int hozon;
  
  sum = cal();
  
  if(sum==0) {
    return depth;
  }
  
  if(depth+sum > l) {
    return -1;
  }
   
  for(i=0;i<4;i++){
    x_2 = x+dx[i];
    y_2 = y+dy[i];
    
    if(p!=-1 && i ==(p+2)%4) continue;

    if(x_2 < 0 || x_2 >= 3 || y_2 < 0 || y_2 >= 3) continue;
 

    hozon=piace[y][x];
    piace[y][x]=piace[y_2][x_2];
    piace[y_2][x_2]=hozon;

 
    res = dfs(x_2,y_2,i,depth+1,l);
     
    if(res!=-1) {
      return res;
    }

    hozon=piace[y][x];
    piace[y][x]=piace[y_2][x_2];
    piace[y_2][x_2]=hozon;
  }
  return -1;
}
 
int cal(){
  int i;
  int hozon;
  int sum=0;

  int z[2]; //絶対値

  for(i = 0;i < 9;i++){

    hozon = piace[i/3][i%3];
    
    if(hozon!=0) {
      hozon--;

      z[0]= i/3 - hozon/3;
      if(z[0]<0)z[0]*=-1;

      z[1]= i%3 - hozon%3;
      if(z[1] < 0)z[1]*=-1;


      sum += z[0] + z[1];
    }
  }
  return sum;
}


