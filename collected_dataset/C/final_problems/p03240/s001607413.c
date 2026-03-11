#include <stdio.h>
int main(){
  int N;
  scanf("%d",&N);
  int x[N];
  int y[N];
  long int h[N];
  long int pre=-1;
  for(int i=0;i<N;i++){
    scanf("%d %d %ld",&x[i],&y[i],&h[i]);
    if(h[i]>pre)pre=h[i];
  }
  int l[2]={-1,0};
  int r[2]={1000,0};

  for(int i=0;i<N;i++){
    if(h[i]==pre){
      if(y[i]>l[0]){
        l[0]=y[i];
        l[1]=i;
      }
      if(x[i]<r[0]){
        r[0]=x[i];
        r[1]=i;
      }
    }
  }
  if((y[r[1]]==0&&x[l[1]]==0) || (y[r[1]]==100 && x[l[1]]==100)) printf("%d %d %ld\n",x[l[1]],y[r[1]],pre)\
;
  else printf("%d %d %ld\n",x[l[1]],y[r[1]],pre+1);
}
