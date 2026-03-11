#include<stdio.h>

void run(void){
  int n,q;
  scanf("%d%d",&n,&q);
  while(q--){
    int v,w;
    scanf("%d%d",&v,&w);
    if(n==1){
      printf("%d\n",v);
      continue;
    }
    while(v!=w){
      if(v>w){
	v=((v-1)+n-1)/n;
      } else {
	w=((w-1)+n-1)/n;
      }
    }
    printf("%d\n",v);
  }
}

int main(void){
  run();
  return 0;
}
