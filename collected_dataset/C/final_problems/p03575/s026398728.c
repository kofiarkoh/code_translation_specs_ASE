#include<stdio.h>

int main(){
  int N;
  int M;
  int a;
  int b;
  int i;
  int j;
  int x;
  int ans = 0;
  int y=0;
  scanf("%d %d",&N,&M);
  int A[50][50]={0};
  int flag[50]={0};
  for(i=0;i<M;i++){
    scanf("%d %d",&a,&b);
    if(a>b){
      A[b-1][a-1]=1;
    }else{
      A[a-1][b-1]=1;
    }
  }
  for(i=0;i<N;i++){
    for(j=0;j<N;j++){
    // printf("%d",A[i][j]);
      	if(A[i][j]>=1){
       	 flag[i]++;
       	 if(A[i][j]==1){
         	 ans++;
         	 y++;
       	 }
       	 if(flag[i]==2){
          	if(A[x][j]==0){
           	 flag[i]=1;
          }else{
            flag[i]=1;
            ans=ans-y;
              y=0;
            A[x][j]=2;
        	}
       	 }
          x = j;
        }
    }
    y=0;
    //printf("  %d\n",flag[i]);
  }
    printf("%d",ans);
    return 0;
  }