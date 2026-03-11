#include <stdio.h>

int search(int, int, int, int[]);

int main(void)
{
  int i;
  int n, s;
  int count;
  int isUsed[10]={0,0,0,0,0,0,0,0,0,0};

  while(scanf("%d %d",&n, &s)!=EOF){
    count=0;
    for(i=0; i<10; i++){
      isUsed[i]=1;
      count+=search(s, n, i, isUsed);
      isUsed[i]=0;
    }
    printf("%d\n", count);
  }
  return 0;
}

int search(int remain, int n, int k, int isUsed[])
{
  int i;
  int count=0;

  remain-=n*k;
  if(remain<0) return 0;
  if(n==1 && remain==0) return 1;

  for(i=0; i<10; i++){
    if(isUsed[i]==1) continue;
    isUsed[i]=1;
    count+=search(remain, n-1, i, isUsed);
    isUsed[i]=0;
  }
  return count;
}