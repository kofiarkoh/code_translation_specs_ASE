#include <stdio.h>
int N,M;
int kazu[100001];
int aka[100001];
int main(){
  int i,j;
  int x,y;
  int ans=0;
  scanf("%d %d",&N,&M);
  for(i=1;i<=N;i++)kazu[i]=1;
  aka[1]=1;
  for(i=0;i<M;i++){
    /*
    for(j=1;j<=N;j++)printf("%d ",kazu[j]);
    printf("\n");
    for(j=1;j<=N;j++)printf("%d ",aka[j]);
    printf("\n");
    */
    scanf("%d %d",&x,&y);
    if(kazu[x]>1){
      if(aka[x]==1){
	kazu[x]--;
	kazu[y]++;
	aka[y]=1;
      }
    }
    else if(kazu[x]==1){
      if(aka[x]==1){
	kazu[x]--;
	kazu[y]++;
	aka[y]=1;
	aka[x]=0;
        
      }      
    }
  }
  for(i=1;i<=N;i++){
    if(aka[i]==1)ans++;
  }
  printf("%d\n",ans);
  return 0;
}
