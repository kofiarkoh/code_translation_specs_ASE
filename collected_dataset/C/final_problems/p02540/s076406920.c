#include <stdio.h>

int main(){
  int N;
  scanf("%d",&N);
  int x[N],y[N],ret[N];
  for(int i = 0;i<N;i++){
	scanf("%d %d",&x[i],&y[i]);
  }
  for(int i = 0;i<N;i++){
   	ret[i] = 1;
    //printf("@1:%d\n",ret[i]);
	for(int j = 0;j<N;j++){
	  if((x[i]<x[j]&&y[i]<y[j])||(x[i]>x[j]&&y[i]>y[j])){
        //printf("i=%d,j=%d\n",i,j);
        if(i<j){
          (ret[i])++;
        }else if(i>j){
          ret[i] = ret[j];
          break;
        }
      }
    }
    //printf("@2:%d\n",ret[i]);
  }
  //printf("###########\n");
  for(int i = 0;i<N;i++){
	printf("%d\n",ret[i]);
  }
  return 0;
}