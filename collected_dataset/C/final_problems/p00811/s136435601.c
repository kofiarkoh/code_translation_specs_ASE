#include <stdio.h>
#include <stdlib.h>
#define num 100000
#define true 1
#define false 0

void isPrime();
int isCheck(int i,int j,int m,int a,int b);
int isbigger(int num1,int num2);
int is1(int i,int j);
int N;
int *prime;

int main(){
  prime = (int *)malloc(num);
  isPrime();
  int m,a,b;
  int i,j,x,y;
  while(scanf("%d %d %d",&m,&a,&b ),(m&&a&&b)){
    int max=0;
    for(i=0;i<N;i++){
      for(j=0;j<N;j++){
        if(isCheck(i,j,m,a,b)) break;
        if(isbigger(max,prime[i]*prime[j])&&is1(i,j)) {
          max = prime[i]*prime[j];
          x = i;
          y = j;
        }
      }
    }
    printf("%d %d\n",prime[x],prime[y] );
  }

  free(prime);
  exit(0);
}

void isPrime(){
  int i,index=0;

  for (int j = 2;j < num;j++){
    for (i = 2;i < j;++i){
      if (j % i == 0){
        break;
      }
    }
    if (j == i){
      *(prime+index) = j;
      index++;
    }
  }
  N = index;
}

int isCheck(int i,int j,int m,int a,int b){
  double min = (double)a/b;
  double real = (double)prime[i]/prime[j];
  int pq = prime[i]*prime[j];
  if(min<=real&&pq<=m)  return false;
  return true;
}

int is1(int i,int j){
  double real = (double)prime[i]/prime[j];
  if (real<=1.0) return true;
  return false;
}

int isbigger(int num1,int num2){
  if (num1 >= num2){
    return false;
  }
  return true;
}

