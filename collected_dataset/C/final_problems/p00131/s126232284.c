#include<stdio.h>
#define N 14

void print_map(int map[N][N]){
  int j,k;

  puts("");
  for(j=0;j<N;j++){
    for(k=0;k<N;k++)
      printf("%d ",map[j][k]);
    puts("");
  }

  return ;
}

void print_ans_map(int map[N][N]){
  int j,k;

  for(j=2;j<N-2;j++){
    for(k=2;k<N-2;k++){
      printf("%d",map[j][k]);
      if(k<N-3)
        printf(" ");
    }
    puts("");
  }

  return ;
}

void copy_map(int map[N][N],int pre_map[N][N]){
  int j,k;

  for(j=0;j<N;j++)
    for(k=0;k<N;k++)
      pre_map[j][k]=map[j][k];
}

int inverse_map3(int map[N][N],int j,int k){
  int count,maxC;
  int p; // 0:situ(jap:sonoba) 1:up 2:down 3:left 4:right

  maxC=0;

  count=0;
  count+=map[j-1][k];
  count+=map[j+1][k];
  count+=map[j][k-1];
  count+=map[j][k+1];
  if(maxC<count){
    maxC=count;
    p=0;
  }

  count=0;
  count+=map[j-1-1][k];
  count+=map[j-1+1][k];
  count+=map[j-1][k-1];
  count+=map[j-1][k+1];
  if(maxC<count){
    maxC=count;
    p=1;
  }

  count=0;
  count+=map[j+1-1][k];
  count+=map[j+1+1][k];
  count+=map[j+1][k-1];
  count+=map[j+1][k+1];
  if(maxC<count){
    maxC=count;
    p=2;
  }

  count=0;
  count+=map[j-1][k-1];
  count+=map[j+1][k-1];
  count+=map[j][k-1-1];
  count+=map[j][k-1+1];
  if(maxC<count){
    maxC=count;
    p=3;
  }

  count=0;
  count+=map[j-1][k+1];
  count+=map[j+1][k+1];
  count+=map[j][k+1-1];
  count+=map[j][k+1+1];
  if(maxC<count){
    p=4;
  }

  return p;
}

void inverse_map2(int map[N][N],int j,int k){
  map[j][k]++;
  map[j-1][k]++;
  map[j+1][k]++;
  map[j][k-1]++;
  map[j][k+1]++;

  map[j][k]%=2;
  map[j-1][k]%=2;
  map[j+1][k]%=2;
  map[j][k-1]%=2;
  map[j][k+1]%=2;
}

void inverse_map_recursion(int map[N][N],int ans_map[N][N],int j,int k){

  if(map[j][k]>0){
    if(inverse_map3(map,j,k)==0 && ans_map[j][k]==0){
      ans_map[j][k]=1;
      inverse_map2(map,j,k);
    }else if(inverse_map3(map,j,k)==1 && ans_map[j-1][k]==0){
      ans_map[j-1][k]=1;
      inverse_map2(map,j-1,k);
    }else if(inverse_map3(map,j,k)==2 && ans_map[j+1][k]==0){
      ans_map[j+1][k]=1;
      inverse_map2(map,j+1,k);
    }else if(inverse_map3(map,j,k)==3 && ans_map[j][k-1]==0){
      ans_map[j][k-1]=1;
      inverse_map2(map,j,k-1);
    }else if(inverse_map3(map,j,k)==4 && ans_map[j][k+1]==0){
      ans_map[j][k+1]=1;
      inverse_map2(map,j,k+1);
    }
  }
  k++;
  if(k>=12){
    k=2;
    j++;
  }
  if(j>=12)
    return ;
  inverse_map_recursion(map,ans_map,j,k);
}

int main(void){
  int i,j,k;
  int n;
  int map[N][N];
  int ans_map[N][N];

  scanf("%d",&n);
  for(i=0;i<n;i++){

    for(j=0;j<N;j++)
      for(k=0;k<N;k++){
        map[j][k]=0;
        ans_map[j][k]=0;
      }

    for(j=2;j<12;j++)
      for(k=2;k<12;k++)
        scanf("%d",&map[j][k]);

    inverse_map_recursion(map,ans_map,2,2);

    print_ans_map(ans_map);
  }

  return 0;
}