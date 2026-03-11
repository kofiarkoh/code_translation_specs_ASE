#include<stdio.h>
#define N 100
int main(){
  int i,j,k=0,num,sum=0,min,list[N][N],tree[N],min_num,tree_num[N];

  scanf("%d",&num);

  for(i=0; i<num; i++){
     tree[i]=0;
    for(j=0; j<num; j++){
      scanf("%d",&list[i][j]);
    }
  }
  min_num = 0;
  tree[k]=0;
  tree_num[k]=1;
  while(k != num-1){
    min = 2001;
    for(i=k; i>=0; i--){
      for(j=0; j<num; j++){
        if(min>list[tree[i]][j] && list[tree[i]][j]>0 && tree_num[j]!=1){
        min = list[tree[i]][j];
        min_num = j;
        }
      }
    }
    k++;
    tree[k] = min_num;
    tree_num[min_num] = 1;
    sum+=min;
  }
  printf("%d\n",sum);
  return 0;
}