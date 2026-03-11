#include <stdio.h>
#include <stdlib.h>
#define WHITE 0
#define GRAY 1
#define BLACK 2
int queue[10000];
int color[10000];
int d[10000];
int M[10000][10000];
int end, top, n;



void enqueue(int n){

 if(end == 10000)
   end = 0;

 queue[end] = n;
 end++;

}


int dequeue(){
int retval;

 if(top == 10000)
  top = 0;

 retval = queue[top];
 top++;



 return retval;
}
int dfs(int s){
int u, maxv = 0;
int count = 0;

enqueue(s);

  for(count = 0; count < n; count++){
   color[count] = WHITE;
   d[count] = 0;
  }

 while(top!=end){
  u = dequeue();
  color[u] = BLACK;
  for(count = 0; count < n; count++){

    if(M[u][count] != 0 && color[count] != BLACK){
      enqueue(count);
      color[count] = GRAY;
      d[count] = d[u] + M[u][count];
      if(d[maxv] < d[count])
        maxv = count;
    }

  }

 }


 return maxv;
}



int main(){
int count, i, j;


scanf("%d", &n);

 for(count = 0; count < n-1; count++){
   scanf("%d %d", &i, &j);
   scanf("%d", &M[i][j]);
   M[j][i] = M[i][j];
 }

  printf("%d\n", d[dfs(dfs(0))]);

 return 0;
}