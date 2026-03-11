#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdbool.h>

int max(int a,int b){if(a>b){return a;}return b;}
int min(int a,int b){if(a<b){return a;}return b;}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}

int dx4[4]={1,-1,0,0};
int dy4[4]={0,0,1,-1};

int f(int x){
  int r=0;
  int a=x/10,b=x%10,k=10;
  while(b!=x){
    r=max(a*b,r);
    b+=(k*(a%10));
    a/=10;k*=10;
  }
  return r;
}

int main(){
  int i,q,n,r;
  scanf("%d",&q);
  for(i=0;i<q;i++){
    scanf("%d",&n);
    r=0;
    while(n>=10){
      n=f(n);
      r++;
    }
    printf("%d\n",r);
  }
  return 0;
}

