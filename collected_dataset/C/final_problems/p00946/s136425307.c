#include<stdio.h>

int main(void){
  int n,m;
  int i,j,k;
  int tmp[200001],hoge[100001];
  int req[100001],fin[200001];
  scanf("%d %d",&n,&m);
  for(i=0;i<m;i++){
    scanf("%d",&req[i]);
  }
 
  for(i=0;i<n;i++){
    tmp[i]=i+1;
  }
  for(i=0;i<n;i++){
    hoge[i]=i+1;
  }
  //  putchar('\n');

  for(k=0;k<n;k++){
    if(hoge[req[m-k-1]-1] && (m-k-1)>=0){
    fin[k]=req[m-k-1];
    printf("%d\n",fin[k]);
    hoge[fin[k]-1]=0;
    tmp[fin[k]-1]=0;
    }
  }
  //  putchar('\n');
  for(k=0;k<n;k++){
    if(tmp[k]){
      printf("%d\n",tmp[k]);
    }
  }
  return 0;
}