#include<stdio.h>

int main(){
  int i,n,t[3];
  int data,kari,hakari;

  while(1){
    scanf("%d%d%d",&t[0],&t[1],&t[2]);
    if(t[0] == 0 || t[1] == 0 || t[2] == 0)
      break;
    scanf("%d",&n);
    if(t[0] >= t[2]){
      t[0] = t[2];
       t[2] = -1;
      }
    if(t[1] >= t[2]&&t[2] != -1)
      t[1] = t[2];
    hakari = t[0] * t[0] + t[1] *t[1];
    for(i = 0; i < n; i++){
      scanf("%d",&data);
      data = data * 2;
      data = data * data;
      if(hakari < data)
	printf("OK\n");
      else 
	printf("NA\n");
    }
  }
  return 0;
}