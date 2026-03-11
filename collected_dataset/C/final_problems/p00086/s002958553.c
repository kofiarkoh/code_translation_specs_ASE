#include <stdio.h>

int main(int argc,char* argv[]){
  int i;
  int graph[101];

  for(;;){
    int from = -1,to = -1,odd_num = 0;

    for(i = 1; i <= 100; i++){
      graph[i] = -1;
    }

    while(scanf("%d %d",&from,&to) != EOF){
      if(from == 0 && to == 0){
	break;
      }else{
	graph[to] = (graph[to] == -1) ? 1 : graph[to]+1;
	graph[from] = (graph[from] == -1) ? 1 : graph[from]+1;
      }
    }

    if(from != 0 || to != 0){
      break;
    }

    for(i = 1; i <= 100; i++){
      if(graph[i] == -1){break;}
      if(graph[i]%2 == 1){odd_num++;}
    }

    if(odd_num == 0 || odd_num == 2){
      printf("OK\n");
    }else{
      printf("NG\n");
    }
  }

  return 0;
  
}