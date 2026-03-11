#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int max(int a,int b){if(a>b){return a;}return b;}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}

int main(){
  int n,s,i,j,r;
  int a[1024];
  while(scanf("%d%d",&n,&s),n!=0){
    for(i=0;i<n;i++){scanf("%d",&a[i]);}
    qsort(a,n,sizeof(int),sortfncsj);
    i=0;j=n-1;r=-1;
    while(i<j){
      if(a[i]+a[j]<=s){r=max(a[i]+a[j],r);i++;}
      else{j--;}
    }
    if(r==-1){printf("NONE\n");}
    else{printf("%d\n",r);}
  }
  return 0;
}

