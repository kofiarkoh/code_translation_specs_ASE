#include<stdio.h>

#define MAX 722

typedef struct {
  int t;
  int m;
  int s;
}Log;

int main(){
  int N; //PC
  int M; //テ・ツュツヲテァツ板?
  Log data[1000];
  int log[MAX];
  int r;
  int Q,T;
  int i,j;


  while(scanf("%d%d",&N,&M),N!=0&&M!=0){
    scanf("%d",&r);
    for(i=0;i<r;i++){
      scanf("%d%d%d%d",&data[i].t,&T,&data[i].m,&data[i].s);
      if(data[i].s==0) data[i].s = -1;
      data[i].t -=540;
  }
    scanf("%d",&Q);
    for(i=0;i<Q;i++){
      int Ts,Te,m;
      int ans=0;
      scanf("%d%d%d",&Ts,&Te,&m);
      Ts-=540;
      Te-=540;
      for(j=0;j<MAX;j++) log[j]=0;
      for(j=0;j<r;j++){
	if(data[j].m==m) log[data[j].t+(data[j].s<0?1:0)] +=data[j].s;
      }
      for(j=1;j<MAX;j++) log[j]+=log[j-1];
      for(j=Ts;j<Te;j++){
	if(log[j]&&log[j+1]) ans++;
      }
      printf("%d\n",ans);
    }
  }
  return 0;
}